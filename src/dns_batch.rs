use std::io;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

// 4KB handles standard DNS (512 bytes) and EDNS0 (commonly up to 4KB).
const DNS_BUFFER_SIZE: usize = 4096;

pub struct BatchDnsSocket {
    socket: UdpSocket,
}

impl BatchDnsSocket {
    pub async fn new(bind_addr: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        Ok(Self { socket })
    }

    pub async fn send_batch(&self, messages: &[(Vec<u8>, SocketAddr)]) -> io::Result<Vec<usize>> {
        if messages.is_empty() {
            return Ok(vec![]);
        }

        let mut sent_lengths = Vec::with_capacity(messages.len());

        for (data, addr) in messages {
            match self.socket.send_to(data, addr).await {
                Ok(len) => sent_lengths.push(len),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Socket became unwritable - return what we've sent so far
                    break;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(sent_lengths)
    }

    pub async fn recv_batch(&self, max_messages: usize) -> io::Result<Vec<(Vec<u8>, SocketAddr)>> {
        // Wait for socket to become readable
        self.socket.readable().await?;

        let mut messages = Vec::with_capacity(max_messages);
        let mut buffer = vec![0u8; DNS_BUFFER_SIZE];

        // Try to receive multiple messages while socket is readable
        for _ in 0..max_messages {
            match self.socket.try_recv_from(&mut buffer) {
                Ok((len, addr)) => {
                    let mut data = vec![0u8; len];
                    data.copy_from_slice(&buffer[..len]);
                    messages.push((data, addr));
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // No more data available
                    break;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_batch_socket_creation() {
        let socket = BatchDnsSocket::new("0.0.0.0:0").await;
        assert!(socket.is_ok());
    }
}
