use std::io;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

#[cfg(target_os = "linux")]
use libc::{c_int, c_void, iovec, msghdr, sockaddr_storage, socklen_t};
#[cfg(target_os = "linux")]
use std::mem::{size_of, zeroed};
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;

// Constants
// 4KB handles standard DNS (512 bytes) and EDNS0 (commonly up to 4KB).
// Reduces allocation from 64MB to 4MB for 1024 messages.
const DNS_BUFFER_SIZE: usize = 4096;
#[cfg(target_os = "linux")]
const MAX_BATCH_SIZE: usize = 1024; // Linux sendmmsg/recvmmsg limit

// FFI declarations for sendmmsg/recvmmsg
#[cfg(target_os = "linux")]
extern "C" {
    fn sendmmsg(sockfd: c_int, msgvec: *mut mmsghdr, vlen: c_int, flags: c_int) -> c_int;

    fn recvmmsg(
        sockfd: c_int,
        msgvec: *mut mmsghdr,
        vlen: c_int,
        flags: c_int,
        timeout: *mut libc::timespec,
    ) -> c_int;
}

#[cfg(target_os = "linux")]
#[repr(C)]
struct mmsghdr {
    msg_hdr: msghdr,
    msg_len: c_int,
}

pub struct BatchDnsSocket {
    socket: UdpSocket,
}

impl BatchDnsSocket {
    pub async fn new(bind_addr: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        Ok(Self { socket })
    }

    #[cfg(target_os = "linux")]
    pub async fn send_batch(&self, messages: &[(Vec<u8>, SocketAddr)]) -> io::Result<Vec<usize>> {
        if messages.is_empty() {
            return Ok(vec![]);
        }

        // Wait for socket to be writable to avoid EWOULDBLOCK on raw syscall
        self.socket.writable().await?;

        let fd = self.socket.as_raw_fd();
        let batch_size = messages.len().min(MAX_BATCH_SIZE);

        // Pre-allocate all vectors with correct capacity
        let mut iovecs: Vec<iovec> = Vec::with_capacity(batch_size);
        let mut addrs: Vec<sockaddr_storage> = Vec::with_capacity(batch_size);
        let mut msgvec: Vec<mmsghdr> = Vec::with_capacity(batch_size);

        // Build structures in one pass with correct pointers
        for (data, addr) in messages.iter().take(batch_size) {
            let mut addr_storage: sockaddr_storage = unsafe { zeroed() };
            let _ = addr_to_sockaddr(addr, &mut addr_storage);

            addrs.push(addr_storage);
            iovecs.push(iovec {
                iov_base: data.as_ptr() as *mut c_void,
                iov_len: data.len(),
            });
        }

        // Build message headers pointing to stable vector elements
        for i in 0..batch_size {
            let mut msg_hdr: msghdr = unsafe { zeroed() };
            msg_hdr.msg_name = &mut addrs[i] as *mut _ as *mut c_void;
            msg_hdr.msg_namelen = size_of::<sockaddr_storage>() as socklen_t;
            msg_hdr.msg_iov = &mut iovecs[i] as *mut iovec;
            msg_hdr.msg_iovlen = 1;

            msgvec.push(mmsghdr {
                msg_hdr,
                msg_len: 0,
            });
        }

        // Send messages
        let result = unsafe { sendmmsg(fd, msgvec.as_mut_ptr(), batch_size as c_int, 0) };

        if result < 0 {
            let err = io::Error::last_os_error();
            // Socket became unwritable between writable() check and sendmmsg call (rare).
            // Return empty batch; caller will retry.
            if err.kind() == io::ErrorKind::WouldBlock {
                return Ok(vec![]);
            }
            return Err(err);
        }

        // Extract sent lengths
        let sent_lengths = msgvec
            .iter()
            .take(result as usize)
            .map(|msg| msg.msg_len as usize)
            .collect();

        Ok(sent_lengths)
    }

    #[cfg(target_os = "linux")]
    pub async fn recv_batch(&self, max_messages: usize) -> io::Result<Vec<(Vec<u8>, SocketAddr)>> {
        // Wait for socket to be readable. The timeout parameter on recvmmsg is ignored
        // for non-blocking sockets, so without this we'd get EAGAIN in a busy loop.
        self.socket.readable().await?;

        let fd = self.socket.as_raw_fd();
        let batch_size = max_messages.min(MAX_BATCH_SIZE);

        // Use 4KB buffers (reduces 64MB to 4MB for 1024 messages).
        // DNS responses are typically <512 bytes, up to 4KB with EDNS0.
        let mut buffers: Vec<Vec<u8>> = vec![vec![0u8; DNS_BUFFER_SIZE]; batch_size];
        let mut iovecs: Vec<iovec> = Vec::with_capacity(batch_size);
        let mut addrs: Vec<sockaddr_storage> = Vec::with_capacity(batch_size);
        let mut msgvec: Vec<mmsghdr> = Vec::with_capacity(batch_size);

        // Build structures in one pass with correct pointers
        for buffer in buffers.iter_mut() {
            addrs.push(unsafe { zeroed() });
            iovecs.push(iovec {
                iov_base: buffer.as_mut_ptr() as *mut c_void,
                iov_len: buffer.len(),
            });
        }

        // Build message headers pointing to stable vector elements
        for i in 0..batch_size {
            let mut msg_hdr: msghdr = unsafe { zeroed() };
            msg_hdr.msg_name = &mut addrs[i] as *mut _ as *mut c_void;
            msg_hdr.msg_namelen = size_of::<sockaddr_storage>() as socklen_t;
            msg_hdr.msg_iov = &mut iovecs[i] as *mut iovec;
            msg_hdr.msg_iovlen = 1;

            msgvec.push(mmsghdr {
                msg_hdr,
                msg_len: 0,
            });
        }

        let result = unsafe {
            recvmmsg(
                fd,
                msgvec.as_mut_ptr(),
                batch_size as c_int,
                0,
                std::ptr::null_mut(), // timeout ignored on non-blocking sockets
            )
        };

        if result < 0 {
            let err = io::Error::last_os_error();
            // Data consumed between readable() check and recvmmsg call (rare race).
            // Return empty batch; caller will retry.
            if err.kind() == io::ErrorKind::WouldBlock {
                return Ok(vec![]);
            }
            return Err(err);
        }

        // Extract received messages - move buffers instead of cloning
        let mut messages = Vec::with_capacity(result as usize);
        for i in 0..result as usize {
            let len = msgvec[i].msg_len as usize;
            let mut buffer = std::mem::take(&mut buffers[i]);
            buffer.truncate(len);

            let addr = sockaddr_to_addr(&addrs[i], msgvec[i].msg_hdr.msg_namelen)?;
            messages.push((buffer, addr));
        }

        Ok(messages)
    }

    // Fallback for non-Linux systems
    #[cfg(not(target_os = "linux"))]
    pub async fn send_batch(&self, messages: &[(Vec<u8>, SocketAddr)]) -> io::Result<Vec<usize>> {
        if messages.is_empty() {
            return Ok(vec![]);
        }

        // Wait for socket to be writable (consistent with Linux version)
        self.socket.writable().await?;

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

    #[cfg(not(target_os = "linux"))]
    pub async fn recv_batch(&self, max_messages: usize) -> io::Result<Vec<(Vec<u8>, SocketAddr)>> {
        // Wait for socket to be readable to avoid EAGAIN busy-wait
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

// Helper functions for address conversion
#[cfg(target_os = "linux")]
fn addr_to_sockaddr(addr: &SocketAddr, storage: &mut sockaddr_storage) -> io::Result<socklen_t> {
    match addr {
        SocketAddr::V4(v4) => {
            let sin = storage as *mut _ as *mut libc::sockaddr_in;
            unsafe {
                (*sin).sin_family = libc::AF_INET as libc::sa_family_t;
                (*sin).sin_port = v4.port().to_be();
                (*sin).sin_addr.s_addr = u32::from_ne_bytes(v4.ip().octets());
            }
            Ok(size_of::<libc::sockaddr_in>() as socklen_t)
        }
        SocketAddr::V6(v6) => {
            let sin6 = storage as *mut _ as *mut libc::sockaddr_in6;
            unsafe {
                (*sin6).sin6_family = libc::AF_INET6 as libc::sa_family_t;
                (*sin6).sin6_port = v6.port().to_be();
                (*sin6).sin6_addr.s6_addr = v6.ip().octets();
                (*sin6).sin6_flowinfo = v6.flowinfo();
                (*sin6).sin6_scope_id = v6.scope_id();
            }
            Ok(size_of::<libc::sockaddr_in6>() as socklen_t)
        }
    }
}

#[cfg(target_os = "linux")]
fn sockaddr_to_addr(storage: &sockaddr_storage, len: socklen_t) -> io::Result<SocketAddr> {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    let family = storage.ss_family;

    match family as i32 {
        libc::AF_INET => {
            if len as usize >= size_of::<libc::sockaddr_in>() {
                let sin = storage as *const _ as *const libc::sockaddr_in;
                let addr = unsafe {
                    let ip = Ipv4Addr::from((*sin).sin_addr.s_addr.to_ne_bytes());
                    let port = u16::from_be((*sin).sin_port);
                    SocketAddr::new(IpAddr::V4(ip), port)
                };
                Ok(addr)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid sockaddr_in length",
                ))
            }
        }
        libc::AF_INET6 => {
            if len as usize >= size_of::<libc::sockaddr_in6>() {
                let sin6 = storage as *const _ as *const libc::sockaddr_in6;
                let addr = unsafe {
                    let ip = Ipv6Addr::from((*sin6).sin6_addr.s6_addr);
                    let port = u16::from_be((*sin6).sin6_port);
                    SocketAddr::new(IpAddr::V6(ip), port)
                };
                Ok(addr)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid sockaddr_in6 length",
                ))
            }
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unknown address family",
        )),
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

    #[cfg(target_os = "linux")]
    #[test]
    fn test_addr_conversion() {
        use std::mem::zeroed;
        use std::net::{IpAddr, Ipv4Addr};

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 53);
        let mut storage: sockaddr_storage = unsafe { zeroed() };

        let len = addr_to_sockaddr(&addr, &mut storage).unwrap();
        let converted = sockaddr_to_addr(&storage, len).unwrap();

        assert_eq!(addr, converted);
    }
}
