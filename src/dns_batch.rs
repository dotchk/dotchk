use std::io;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

#[cfg(target_os = "linux")]
use libc::{c_int, c_void, iovec, msghdr, sockaddr_storage, socklen_t};
#[cfg(target_os = "linux")]
use std::mem::{size_of, zeroed};
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;

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

        let fd = self.socket.as_raw_fd();
        let batch_size = messages.len().min(1024); // Linux typically supports up to 1024

        // Prepare message headers
        let mut msgvec: Vec<mmsghdr> = Vec::with_capacity(batch_size);
        let mut iovecs: Vec<iovec> = Vec::with_capacity(batch_size);
        let mut addrs: Vec<sockaddr_storage> = Vec::with_capacity(batch_size);

        for (data, addr) in messages.iter().take(batch_size) {
            let mut addr_storage: sockaddr_storage = unsafe { zeroed() };
            let addr_len = addr_to_sockaddr(addr, &mut addr_storage)?;

            let iov = iovec {
                iov_base: data.as_ptr() as *mut c_void,
                iov_len: data.len(),
            };

            let mut msg_hdr: msghdr = unsafe { zeroed() };
            msg_hdr.msg_name = &mut addr_storage as *mut _ as *mut c_void;
            msg_hdr.msg_namelen = addr_len;
            msg_hdr.msg_iov = &iov as *const _ as *mut iovec;
            msg_hdr.msg_iovlen = 1;

            iovecs.push(iov);
            addrs.push(addr_storage);

            msgvec.push(mmsghdr {
                msg_hdr,
                msg_len: 0,
            });
        }

        // Fix pointers after vectors are populated
        for (i, (msg, addr)) in msgvec.iter_mut().zip(addrs.iter_mut()).enumerate() {
            msg.msg_hdr.msg_name = addr as *mut _ as *mut c_void;
            msg.msg_hdr.msg_iov = &mut iovecs[i] as *mut iovec;
        }

        // Send messages
        let result = unsafe { sendmmsg(fd, msgvec.as_mut_ptr(), batch_size as c_int, 0) };

        if result < 0 {
            return Err(io::Error::last_os_error());
        }

        // Extract sent lengths
        let mut sent_lengths = Vec::with_capacity(result as usize);
        for msg in msgvec.iter().take(result as usize) {
            sent_lengths.push(msg.msg_len as usize);
        }

        Ok(sent_lengths)
    }

    #[cfg(target_os = "linux")]
    pub async fn recv_batch(&self, max_messages: usize) -> io::Result<Vec<(Vec<u8>, SocketAddr)>> {
        let fd = self.socket.as_raw_fd();
        let batch_size = max_messages.min(1024);

        // Prepare buffers and headers
        let mut buffers: Vec<Vec<u8>> = vec![vec![0u8; 65535]; batch_size];
        let mut msgvec: Vec<mmsghdr> = Vec::with_capacity(batch_size);
        let mut iovecs: Vec<iovec> = Vec::with_capacity(batch_size);
        let mut addrs: Vec<sockaddr_storage> = Vec::with_capacity(batch_size);

        for buffer in buffers.iter_mut().take(batch_size) {
            let mut addr_storage: sockaddr_storage = unsafe { zeroed() };

            let iov = iovec {
                iov_base: buffer.as_mut_ptr() as *mut c_void,
                iov_len: buffer.len(),
            };

            let mut msg_hdr: msghdr = unsafe { zeroed() };
            msg_hdr.msg_name = &mut addr_storage as *mut _ as *mut c_void;
            msg_hdr.msg_namelen = size_of::<sockaddr_storage>() as socklen_t;
            msg_hdr.msg_iov = &iov as *const _ as *mut iovec;
            msg_hdr.msg_iovlen = 1;

            iovecs.push(iov);
            addrs.push(addr_storage);

            msgvec.push(mmsghdr {
                msg_hdr,
                msg_len: 0,
            });
        }

        // Fix pointers after vectors are populated
        for (i, (msg, addr)) in msgvec.iter_mut().zip(addrs.iter_mut()).enumerate() {
            msg.msg_hdr.msg_name = addr as *mut _ as *mut c_void;
            msg.msg_hdr.msg_iov = &mut iovecs[i] as *mut iovec;
        }

        // Receive messages with timeout
        let mut timeout = libc::timespec {
            tv_sec: 1,
            tv_nsec: 0,
        };

        let result = unsafe {
            recvmmsg(
                fd,
                msgvec.as_mut_ptr(),
                batch_size as c_int,
                0,
                &mut timeout,
            )
        };

        if result < 0 {
            return Err(io::Error::last_os_error());
        }

        // Extract received messages
        let mut messages = Vec::with_capacity(result as usize);
        for i in 0..result as usize {
            let len = msgvec[i].msg_len as usize;
            buffers[i].truncate(len);

            let addr = sockaddr_to_addr(&addrs[i], msgvec[i].msg_hdr.msg_namelen)?;
            messages.push((buffers[i].clone(), addr));
        }

        Ok(messages)
    }

    // Fallback for non-Linux systems
    #[cfg(not(target_os = "linux"))]
    pub async fn send_batch(&self, messages: &[(Vec<u8>, SocketAddr)]) -> io::Result<Vec<usize>> {
        let mut sent_lengths = Vec::with_capacity(messages.len());

        for (data, addr) in messages {
            match self.socket.send_to(data, addr).await {
                Ok(len) => sent_lengths.push(len),
                Err(e) => return Err(e),
            }
        }

        Ok(sent_lengths)
    }

    #[cfg(not(target_os = "linux"))]
    pub async fn recv_batch(&self, max_messages: usize) -> io::Result<Vec<(Vec<u8>, SocketAddr)>> {
        let mut messages = Vec::with_capacity(max_messages);
        let mut buffer = vec![0u8; 65535];

        // Use tokio::select! with timeout for batch receiving
        use tokio::time::{timeout, Duration};

        for _ in 0..max_messages {
            match timeout(
                Duration::from_millis(10),
                self.socket.recv_from(&mut buffer),
            )
            .await
            {
                Ok(Ok((len, addr))) => {
                    let mut data = vec![0u8; len];
                    data.copy_from_slice(&buffer[..len]);
                    messages.push((data, addr));
                }
                _ => break,
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
