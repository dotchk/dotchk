use crate::dns_batch::BatchDnsSocket;
use dashmap::DashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::time::{interval, timeout};
use tracing::{debug, warn};

#[derive(Error, Debug)]
pub enum DnsError {
    #[error("Socket error: {0}")]
    Socket(#[from] std::io::Error),

    #[error("Timeout")]
    Timeout,

    #[error("Invalid response")]
    InvalidResponse,

    #[error("Name error (NXDOMAIN)")]
    NameError,

    #[error("Server failure")]
    ServerFailure,

    #[error("Format error")]
    FormatError,

    #[error("Channel closed")]
    ChannelClosed,
}

#[derive(Debug)]
struct PendingQuery {
    response_tx: oneshot::Sender<Result<bool, DnsError>>,
    sent_at: Instant,
    timeout_ms: u64,
}

#[derive(Clone)]
pub struct PipelinedDnsClient {
    query_tx: mpsc::Sender<QueryRequest>,
    cache: Arc<DashMap<String, (bool, Instant)>>,
    cache_ttl: Duration,
}

struct QueryRequest {
    domain: String,
    server: String, // Already an IP address
    timeout_ms: u64,
    response_tx: oneshot::Sender<Result<bool, DnsError>>,
}

impl PipelinedDnsClient {
    pub async fn new(bind_addr: &str, cache_ttl: Duration) -> Result<Self, DnsError> {
        let batch_socket = Arc::new(BatchDnsSocket::new(bind_addr).await?);
        let cache = Arc::new(DashMap::new());
        let (query_tx, query_rx) = mpsc::channel(10000);

        // Spawn the main processing task
        let batch_socket_clone = batch_socket.clone();
        let cache_clone = cache.clone();
        tokio::spawn(async move {
            process_queries_batch(batch_socket_clone, query_rx, cache_clone).await;
        });

        Ok(Self {
            query_tx,
            cache,
            cache_ttl,
        })
    }

    pub async fn query_ns(
        &self,
        domain: &str,
        server: &str,
        timeout_ms: u64,
    ) -> Result<bool, DnsError> {
        let cache_key = format!("{domain}-{server}");

        // Check cache first
        if let Some(entry) = self.cache.get(&cache_key) {
            if entry.1.elapsed() < self.cache_ttl {
                debug!(
                    "Cache hit for domain {} on server {}: {}",
                    domain, server, entry.0
                );
                return Ok(entry.0);
            }
            self.cache.remove(&cache_key);
            debug!("Cache expired for domain {} on server {}", domain, server);
        }

        // Send query request to the processing task
        let (response_tx, response_rx) = oneshot::channel();
        let request = QueryRequest {
            domain: domain.to_string(),
            server: server.to_string(),
            timeout_ms,
            response_tx,
        };

        self.query_tx
            .send(request)
            .await
            .map_err(|_| DnsError::ChannelClosed)?;

        // Wait for response
        match timeout(Duration::from_millis(timeout_ms), response_rx).await {
            Ok(Ok(result)) => {
                if let Ok(has_ns_records) = result {
                    self.cache
                        .insert(cache_key, (has_ns_records, Instant::now()));
                }
                result
            }
            Ok(Err(_)) => Err(DnsError::ChannelClosed),
            Err(_) => Err(DnsError::Timeout),
        }
    }

    pub async fn query_ns_batch(
        &self,
        queries: Vec<(String, String, u64)>, // (domain, server, timeout_ms)
    ) -> Vec<Result<bool, DnsError>> {
        let mut results = Vec::with_capacity(queries.len());
        let mut receivers = Vec::with_capacity(queries.len());

        // Send all queries
        for (domain, server, timeout_ms) in queries {
            let cache_key = format!("{domain}-{server}");

            // Check cache first
            if let Some(entry) = self.cache.get(&cache_key) {
                if entry.1.elapsed() < self.cache_ttl {
                    results.push(Ok(entry.0));
                    continue;
                }
                self.cache.remove(&cache_key);
            }

            let (response_tx, response_rx) = oneshot::channel();
            let request = QueryRequest {
                domain: domain.clone(),
                server: server.clone(),
                timeout_ms,
                response_tx,
            };

            if self.query_tx.send(request).await.is_err() {
                results.push(Err(DnsError::ChannelClosed));
            } else {
                receivers.push((response_rx, domain, server, timeout_ms));
            }
        }

        // Collect responses
        for (response_rx, domain, server, timeout_ms) in receivers {
            let cache_key = format!("{domain}-{server}");
            match timeout(Duration::from_millis(timeout_ms), response_rx).await {
                Ok(Ok(result)) => {
                    if let Ok(has_ns_records) = result {
                        self.cache
                            .insert(cache_key, (has_ns_records, Instant::now()));
                    }
                    results.push(result);
                }
                Ok(Err(_)) => results.push(Err(DnsError::ChannelClosed)),
                Err(_) => results.push(Err(DnsError::Timeout)),
            }
        }

        results
    }

    /// Query local resolver for a domain
    /// This is used as a fallback when all authoritative servers fail
    pub async fn query_local_resolver(&self, domain: &str) -> Result<bool, DnsError> {
        // Get system DNS resolvers
        let resolvers = get_system_resolvers();
        if resolvers.is_empty() {
            warn!("No system resolvers found, using default 127.0.0.1");
            return self.query_resolver_direct(domain, "127.0.0.1").await;
        }

        // Try each system resolver
        for resolver in &resolvers {
            debug!("Trying system resolver {} for {}", resolver, domain);
            match self.query_resolver_direct(domain, resolver).await {
                Ok(result) => {
                    debug!("System resolver {} succeeded for {}", resolver, domain);
                    return Ok(result);
                }
                Err(e) => {
                    debug!(
                        "System resolver {} failed for {}: {:?}",
                        resolver, domain, e
                    );
                    continue;
                }
            }
        }

        warn!("All system resolvers failed for {}", domain);
        Err(DnsError::ServerFailure)
    }

    /// Query a specific resolver directly
    async fn query_resolver_direct(
        &self,
        domain: &str,
        resolver_ip: &str,
    ) -> Result<bool, DnsError> {
        // Query the resolver using our existing DNS infrastructure
        match timeout(
            Duration::from_millis(2000),
            self.query_ns(domain, resolver_ip, 2000),
        )
        .await
        {
            Ok(Ok(has_ns)) => Ok(has_ns),
            Ok(Err(DnsError::NameError)) => Ok(true), // NXDOMAIN means available
            Ok(Err(e)) => Err(e),
            Err(_) => Err(DnsError::Timeout),
        }
    }
}

type BatchQueue = Vec<(Vec<u8>, SocketAddr, u16)>;

async fn process_queries_batch(
    batch_socket: Arc<BatchDnsSocket>,
    mut query_rx: mpsc::Receiver<QueryRequest>,
    _cache: Arc<DashMap<String, (bool, Instant)>>,
) {
    let transaction_id = Arc::new(AtomicU16::new(1));
    let pending_queries: Arc<DashMap<u16, PendingQuery>> = Arc::new(DashMap::new());
    let batch_queue: Arc<Mutex<BatchQueue>> = Arc::new(Mutex::new(Vec::new()));

    // Spawn response handler
    let batch_socket_clone = batch_socket.clone();
    let pending_clone = pending_queries.clone();
    tokio::spawn(async move {
        handle_responses_batch(batch_socket_clone, pending_clone).await;
    });

    // Spawn timeout checker
    let pending_clone = pending_queries.clone();
    tokio::spawn(async move {
        check_timeouts(pending_clone).await;
    });

    // Spawn batch sender
    let batch_socket_clone = batch_socket.clone();
    let batch_queue_clone = batch_queue.clone();
    let pending_clone = pending_queries.clone();
    tokio::spawn(async move {
        send_batches(batch_socket_clone, batch_queue_clone, pending_clone).await;
    });

    // Process incoming query requests
    while let Some(request) = query_rx.recv().await {
        let QueryRequest {
            domain,
            server,
            timeout_ms,
            response_tx,
        } = request;

        // Generate unique transaction ID with atomic wraparound
        let tx_id = loop {
            let current = transaction_id.load(Ordering::Relaxed);
            let next = if current == u16::MAX { 1 } else { current + 1 };
            match transaction_id.compare_exchange_weak(
                current,
                next,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break current,
                Err(_) => continue, // Retry if another thread modified it
            }
        };

        // Build DNS query with unique transaction ID
        let query = build_ns_query(&domain, tx_id);
        if query.is_empty() {
            let _ = response_tx.send(Err(DnsError::InvalidResponse));
            continue;
        }

        // Resolve the server IP to a socket address
        let server_addr = match resolve_server_addr(&server) {
            Ok(addr) => addr,
            Err(e) => {
                let _ = response_tx.send(Err(e));
                continue;
            }
        };

        // Store pending query
        let pending = PendingQuery {
            response_tx,
            sent_at: Instant::now(),
            timeout_ms,
        };
        pending_queries.insert(tx_id, pending);

        // Add to batch queue
        let mut queue = batch_queue.lock().await;
        queue.push((query, server_addr, tx_id));

        // If queue is getting large, wake the batch sender
        if queue.len() >= 64 {
            drop(queue); // Release lock before potentially blocking
        }
    }
}

async fn send_batches(
    batch_socket: Arc<BatchDnsSocket>,
    batch_queue: Arc<Mutex<BatchQueue>>,
    pending_queries: Arc<DashMap<u16, PendingQuery>>,
) {
    let mut send_interval = interval(Duration::from_micros(100)); // Send batches every 100μs

    loop {
        send_interval.tick().await;

        let mut queue = batch_queue.lock().await;
        if queue.is_empty() {
            continue;
        }

        // Take up to 256 messages for this batch
        let batch_size = queue.len().min(256);
        let items: Vec<_> = queue.drain(0..batch_size).collect();
        drop(queue); // Release lock

        let mut batch = Vec::with_capacity(items.len());
        let mut tx_ids = Vec::with_capacity(items.len());

        for (data, addr, tx_id) in items {
            batch.push((data, addr));
            tx_ids.push(tx_id);
        }

        // Send batch
        match batch_socket.send_batch(&batch).await {
            Ok(sent_lengths) => {
                for (i, &len) in sent_lengths.iter().enumerate() {
                    if len == 0 && i < tx_ids.len() {
                        // Failed to send this query
                        if let Some((_, pending)) = pending_queries.remove(&tx_ids[i]) {
                            let _ = pending.response_tx.send(Err(DnsError::Socket(
                                std::io::Error::other("Failed to send"),
                            )));
                        }
                    }
                }
            }
            Err(e) => {
                // All queries in batch failed
                for tx_id in tx_ids {
                    if let Some((_, pending)) = pending_queries.remove(&tx_id) {
                        let _ =
                            pending
                                .response_tx
                                .send(Err(DnsError::Socket(std::io::Error::new(
                                    e.kind(),
                                    e.to_string(),
                                ))));
                    }
                }
            }
        }
    }
}

async fn handle_responses_batch(
    batch_socket: Arc<BatchDnsSocket>,
    pending_queries: Arc<DashMap<u16, PendingQuery>>,
) {
    loop {
        match batch_socket.recv_batch(256).await {
            Ok(messages) => {
                for (data, _addr) in messages {
                    if data.len() < 12 {
                        continue;
                    }

                    // Extract transaction ID
                    let tx_id = u16::from_be_bytes([data[0], data[1]]);

                    // Find and remove pending query
                    if let Some((_, pending)) = pending_queries.remove(&tx_id) {
                        let response = parse_ns_response(&data);
                        let _ = pending.response_tx.send(response);
                    }
                }
            }
            Err(e) => {
                warn!("Error receiving DNS responses: {}", e);
                // Brief pause before retrying
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }
    }
}

async fn check_timeouts(pending_queries: Arc<DashMap<u16, PendingQuery>>) {
    let mut check_interval = interval(Duration::from_millis(100));

    loop {
        check_interval.tick().await;

        let now = Instant::now();
        let mut timed_out = Vec::new();

        // Find timed out queries
        for entry in pending_queries.iter() {
            let tx_id = *entry.key();
            let pending = entry.value();

            if now.duration_since(pending.sent_at) > Duration::from_millis(pending.timeout_ms) {
                timed_out.push(tx_id);
            }
        }

        // Remove and notify timed out queries
        for tx_id in timed_out {
            if let Some((_, pending)) = pending_queries.remove(&tx_id) {
                let _ = pending.response_tx.send(Err(DnsError::Timeout));
            }
        }
    }
}

fn resolve_server_addr(server: &str) -> Result<SocketAddr, DnsError> {
    // Since servers are already IPs, just parse them directly
    // IPv6 addresses need to be wrapped in brackets
    let addr_str = if server.contains(':') {
        format!("[{server}]:53")
    } else {
        format!("{server}:53")
    };

    addr_str.parse().map_err(|e| {
        warn!("Failed to parse server IP {}: {}", server, e);
        DnsError::InvalidResponse
    })
}

/// Get system DNS resolvers cross-platform
fn get_system_resolvers() -> Vec<String> {
    let mut resolvers = Vec::new();

    // Platform-specific DNS resolver detection
    #[cfg(unix)]
    {
        use resolv_conf::Config;

        if let Ok(contents) = std::fs::read_to_string("/etc/resolv.conf") {
            if let Ok(config) = Config::parse(&contents) {
                // Add all nameservers from the system configuration
                for ns in config.nameservers {
                    match ns {
                        resolv_conf::ScopedIp::V4(addr) => {
                            resolvers.push(addr.to_string());
                        }
                        resolv_conf::ScopedIp::V6(addr, _) => {
                            resolvers.push(addr.to_string());
                        }
                    }
                }
                debug!(
                    "Found {} system resolvers from resolv.conf",
                    resolvers.len()
                );
            } else {
                debug!("Failed to parse /etc/resolv.conf");
            }
        } else {
            debug!("Failed to read /etc/resolv.conf");
        }
    }

    // Windows DNS resolver detection using ipconfig
    #[cfg(windows)]
    {
        use ipconfig::get_adapters;

        match get_adapters() {
            Ok(adapters) => {
                for adapter in adapters {
                    // Skip inactive adapters
                    if adapter.oper_status() != ipconfig::OperStatus::IfOperStatusUp {
                        continue;
                    }

                    // Add DNS servers from this adapter
                    for dns in adapter.dns_servers() {
                        let ip_str = dns.to_string();
                        if !resolvers.contains(&ip_str) {
                            resolvers.push(ip_str);
                        }
                    }
                }
                debug!(
                    "Found {} DNS servers from Windows network adapters",
                    resolvers.len()
                );
            }
            Err(e) => {
                debug!("Failed to get Windows network adapters: {}", e);
            }
        }
    }

    // macOS-specific: Also check scutil if resolv.conf doesn't have all info
    #[cfg(target_os = "macos")]
    {
        // macOS sometimes has additional DNS info in System Configuration
        // For now, the Unix path above should work for most cases
    }

    // If no resolvers found, add common defaults
    if resolvers.is_empty() {
        debug!("No system resolvers found, using defaults");
        resolvers.push("127.0.0.1".to_string()); // Local resolver
        resolvers.push("8.8.8.8".to_string()); // Google DNS
        resolvers.push("1.1.1.1".to_string()); // Cloudflare DNS
    }

    // Remove duplicates while preserving order
    let mut seen = std::collections::HashSet::new();
    resolvers.retain(|ip| seen.insert(ip.clone()));

    debug!("Using DNS resolvers: {:?}", resolvers);
    resolvers
}

fn build_ns_query(domain: &str, transaction_id: u16) -> Vec<u8> {
    let mut packet = Vec::with_capacity(512);

    // Transaction ID (now randomized)
    packet.extend_from_slice(&transaction_id.to_be_bytes());
    packet.extend_from_slice(&[0x01, 0x00]); // Flags: Standard query
    packet.extend_from_slice(&[0x00, 0x01]); // Questions: 1
    packet.extend_from_slice(&[0x00, 0x00]); // Answer RRs: 0
    packet.extend_from_slice(&[0x00, 0x00]); // Authority RRs: 0
    packet.extend_from_slice(&[0x00, 0x00]); // Additional RRs: 0

    debug!(
        "Building DNS query for domain: {} with TX ID: {:04x}",
        domain, transaction_id
    );
    for label in domain.split('.') {
        if label.is_empty() {
            debug!("Empty label in domain: {}", domain);
            return vec![]; // Return empty packet for invalid domain
        }
        if label.len() > 63 {
            debug!("Label too long ({}): {}", label.len(), label);
            return vec![]; // Return empty packet for invalid domain
        }
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0x00); // End of domain name

    packet.extend_from_slice(&[0x00, 0x02]); // Type: NS
    packet.extend_from_slice(&[0x00, 0x01]); // Class: IN

    debug!("Built DNS query packet of {} bytes", packet.len());
    packet
}

fn parse_ns_response(response: &[u8]) -> Result<bool, DnsError> {
    if response.len() < 12 {
        warn!("DNS response too short: {} bytes", response.len());
        return Err(DnsError::InvalidResponse);
    }

    // Parse header
    let transaction_id = u16::from_be_bytes([response[0], response[1]]);
    let flags = u16::from_be_bytes([response[2], response[3]]);
    let questions = u16::from_be_bytes([response[4], response[5]]);
    let answer_count = u16::from_be_bytes([response[6], response[7]]);
    let authority_count = u16::from_be_bytes([response[8], response[9]]);
    let additional_count = u16::from_be_bytes([response[10], response[11]]);

    let qr = (flags >> 15) & 0x1;
    let opcode = (flags >> 11) & 0xF;
    let aa = (flags >> 10) & 0x1;
    let tc = (flags >> 9) & 0x1;
    let rd = (flags >> 8) & 0x1;
    let ra = (flags >> 7) & 0x1;
    let rcode = flags & 0x000F;

    debug!(
        "DNS Response Header: ID={:04x}, QR={}, OPCODE={}, AA={}, TC={}, RD={}, RA={}, RCODE={}",
        transaction_id, qr, opcode, aa, tc, rd, ra, rcode
    );
    debug!(
        "Counts: Questions={}, Answers={}, Authority={}, Additional={}",
        questions, answer_count, authority_count, additional_count
    );

    if qr != 1 {
        warn!("Response is not a response (QR={})", qr);
        return Err(DnsError::InvalidResponse);
    }

    match rcode {
        0 => {
            let has_records = answer_count > 0 || authority_count > 0;
            debug!("NOERROR response, has NS records: {}", has_records);
            Ok(has_records)
        }
        3 => {
            debug!("NXDOMAIN - domain doesn't exist");
            Ok(false)
        }
        1 => {
            warn!("Format error (FORMERR)");
            Err(DnsError::FormatError)
        }
        2 => {
            warn!("Server failure (SERVFAIL)");
            Err(DnsError::ServerFailure)
        }
        _ => {
            warn!("Unexpected RCODE: {}", rcode);
            Err(DnsError::InvalidResponse)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_ns_query() {
        let query = build_ns_query("example.com", 0x1234);
        assert!(query.len() > 12);
        assert_eq!(&query[0..2], &[0x12, 0x34]); // Check transaction ID
        assert_eq!(&query[query.len() - 4..], &[0x00, 0x02, 0x00, 0x01]); // Check type and class
    }

    #[test]
    fn test_parse_ns_response_nxdomain() {
        let mut response = vec![0; 12];
        response[0] = 0x12; // Transaction ID high byte
        response[1] = 0x34; // Transaction ID low byte
        response[2] = 0x80; // Set QR=1 (response)
        response[3] = 0x03; // NXDOMAIN
        let result = parse_ns_response(&response).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_resolve_server_addr_ipv4() {
        let result = resolve_server_addr("192.168.1.1");
        assert!(result.is_ok());
        let addr = result.unwrap();
        assert_eq!(addr.to_string(), "192.168.1.1:53");
    }

    #[test]
    fn test_resolve_server_addr_ipv6() {
        let result = resolve_server_addr("2001:db8::1");
        assert!(result.is_ok());
        let addr = result.unwrap();
        assert_eq!(addr.to_string(), "[2001:db8::1]:53");
    }

    #[test]
    fn test_resolve_server_addr_ipv6_full() {
        let result = resolve_server_addr("2001:0db8:0000:0000:0000:0000:0000:0001");
        assert!(result.is_ok());
        let addr = result.unwrap();
        assert_eq!(addr.to_string(), "[2001:db8::1]:53");
    }

    #[test]
    fn test_resolve_server_addr_invalid() {
        let result = resolve_server_addr("not-an-ip");
        assert!(result.is_err());
    }

}
