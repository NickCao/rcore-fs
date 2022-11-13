//! Abstraction for underlying communication and storage
//!
//! For a distributed filesystem, communication and storage
//! are equally important, and in many times, closely related.
//! In order to utilize the full potential of RDMA capable networks
//! and NVME drives, the abstraction is designed around the concept
//! of Remote Memory Access and NVME-like block storage, with graceful
//! fallback when these features are not available.
//!
//! In this abstraction, every node is assigned a unique node id
//! of 64 bits, and each node exposes a virtual address space of
//! additional 64 bits. Together, the 128 bit address uniquely
//! identifies a single block of storage, the address length is
//! carefully designed so that it could fit into an IPv6 address,
//! allowing the co-design of networking and storage.
//!
//! On each block, two operations MUST be implemented, get and set,
//! or, read and write. These operations MUST atomically observe or modify
//! the content of the specified block. And for easier implementation
//! of distributed filesystems around this abstraction, two additional
//! operations SHOULD be implemented, discard and CAS. Discard marks a
//! block as no longer needed, so that it's backing storage can be
//! garbage collected, in the absence of the discard operation, blocks
//! of all zero MAY be treated as empty blocks and discarded, however
//! it MUST be ensured that new blocked are initialized with zeros on
//! first access. CAS denotes compare and swap, it ensures data
//! consistency when a block is concurrently modified by multiple nodes.
//!
//! Above the abstraction, the whole communication and storage system can
//! be seen as a huge NVME drive, with unique characteristics reguarding
//! the performance and consistency of accessing local or remote blocks.
extern crate alloc;
use alloc::string::String;

/// Abstract transport
pub trait Transport: Send + Sync {
    /// get self node id
    fn nid(&self) -> u64;
    /// get total number of nodes
    fn len(&self) -> u64;
    /// get block by block id
    fn get(&self, nid: u64, bid: u64, buf: &mut [u8]) -> Result<usize, String>;
    /// set block by block id
    fn set(&self, nid: u64, bid: u64, buf: &[u8]) -> Result<(), String>;
    /// allocate an unused block id
    fn next(&self) -> u64;
}

#[cfg(feature = "std")]
/// Loopback transport
pub mod loopback {
    use crate::Transport;
    use alloc::string::String;
    use rand::RngCore;
    use std::{
        collections::HashMap,
        io::{Read, Write},
        net::{Ipv4Addr, SocketAddr, SocketAddrV4},
        sync::{Arc, Mutex},
        usize,
    };

    /// A transport based on TCP over loopback interface
    pub struct LoopbackTransport {
        nid: u64,
        len: u64,
        base: u16,
        store: Arc<Mutex<HashMap<u64, Vec<u8>>>>,
        _handle: std::thread::JoinHandle<()>,
    }

    impl LoopbackTransport {
        /// create transport from nid, len and base port number,
        /// node ports are allocated sequentially
        pub fn new(nid: u64, len: u64, base: u16) -> Result<Self, String> {
            let listener = std::net::TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::new(127, 0, 0, 1),
                base + nid as u16,
            )))
            .unwrap();
            let store: Arc<Mutex<HashMap<u64, Vec<u8>>>> = Default::default();
            let handle = {
                let store = store.clone();
                std::thread::spawn(move || {
                    for stream in listener.incoming() {
                        let mut stream = stream.unwrap();

                        let mut op = 0u64.to_be_bytes();
                        stream.read_exact(&mut op).unwrap();
                        let op = u64::from_be_bytes(op);

                        match op {
                            0 => {
                                let mut bid = 0u64.to_be_bytes();
                                stream.read_exact(&mut bid).unwrap();
                                let bid = u64::from_be_bytes(bid);
                                if let Some(msg) = store.lock().unwrap().get(&bid) {
                                    stream.write_all(&msg.len().to_be_bytes()).unwrap();
                                    stream.write_all(msg).unwrap();
                                } else {
                                    stream.write_all(&0u64.to_be_bytes()).unwrap();
                                }
                            }
                            1 => {
                                let mut bid = 0u64.to_be_bytes();
                                stream.read_exact(&mut bid).unwrap();
                                let bid = u64::from_be_bytes(bid);
                                let mut len = 0u64.to_be_bytes();
                                stream.read_exact(&mut len).unwrap();
                                let len = u64::from_be_bytes(len);
                                let mut buf = vec![0u8; len as usize];
                                stream.read_exact(&mut buf).unwrap();
                                store.lock().unwrap().insert(bid, buf);
                                stream.write_all(&0u64.to_be_bytes()).unwrap();
                            }
                            _ => unreachable!(),
                        }
                    }
                })
            };
            Ok(Self {
                nid,
                len,
                base,
                store,
                _handle: handle,
            })
        }
    }

    impl Transport for LoopbackTransport {
        fn nid(&self) -> u64 {
            self.nid
        }
        fn len(&self) -> u64 {
            self.len
        }
        fn get(&self, nid: u64, bid: u64, mut buf: &mut [u8]) -> Result<usize, String> {
            if nid == self.nid {
                if let Some(msg) = self.store.lock().unwrap().get(&bid) {
                    Ok(buf.write(&msg).unwrap())
                } else {
                    Err("bid not found".to_string())
                }
            } else {
                let mut stream = std::net::TcpStream::connect(SocketAddr::V4(SocketAddrV4::new(
                    Ipv4Addr::new(127, 0, 0, 1),
                    self.base + nid as u16,
                )))
                .unwrap();
                // opcode 0 for get
                stream.write_all(&0u64.to_be_bytes()).unwrap();
                // bid
                stream.write_all(&bid.to_be_bytes()).unwrap();
                // buf len
                let mut len = 0u64.to_be_bytes();
                stream.read_exact(&mut len).unwrap();
                let len = u64::from_be_bytes(len) as usize;
                if len == 0 {
                    return Err("bid not found".to_string());
                }
                if len > buf.len() {
                    return Err("buffer too small".to_string());
                }
                // buf
                stream.read_exact(&mut buf[..len]).unwrap();
                Ok(len)
            }
        }
        fn set(&self, nid: u64, bid: u64, buf: &[u8]) -> Result<(), String> {
            if nid == self.nid {
                self.store.lock().unwrap().insert(bid, buf.to_vec());
                Ok(())
            } else {
                let mut stream = std::net::TcpStream::connect(SocketAddr::V4(SocketAddrV4::new(
                    Ipv4Addr::new(127, 0, 0, 1),
                    self.base + nid as u16,
                )))
                .unwrap();
                // opcode 1 for set
                stream.write_all(&1u64.to_be_bytes()).unwrap();
                // bid
                stream.write_all(&bid.to_be_bytes()).unwrap();
                // buf len
                stream.write_all(&buf.len().to_be_bytes()).unwrap();
                // buf
                stream.write_all(&buf).unwrap();
                // ack, TODO: check value of ack
                let mut ack = 0u64.to_be_bytes();
                stream.read_exact(&mut ack).unwrap();
                Ok(())
            }
        }
        fn next(&self) -> u64 {
            rand::thread_rng().next_u64()
        }
    }

    #[cfg(test)]
    mod test {
        use crate::transport::{loopback::LoopbackTransport, Transport};
        #[test]
        fn transport() {
            let t1 = LoopbackTransport::new(0, 2, 3000).unwrap();
            let t2 = LoopbackTransport::new(1, 2, 3000).unwrap();
            t1.set(0, 1, b"foo").unwrap();
            t1.set(1, 1, b"bar").unwrap();
            t2.set(0, 2, b"baz").unwrap();
            let mut buf = vec![0u8; 4096];
            let n = t1.get(0, 1, &mut buf).unwrap();
            assert_eq!(b"foo", &buf[..n]);
            let n = t1.get(1, 1, &mut buf).unwrap();
            assert_eq!(b"bar", &buf[..n]);
            let n = t1.get(0, 2, &mut buf).unwrap();
            assert_eq!(b"baz", &buf[..n]);
        }
    }
}
