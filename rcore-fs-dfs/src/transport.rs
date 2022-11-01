use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::{Arc, Mutex},
};

pub trait Transport {
    /// get node id
    fn id(&mut self) -> usize;
    /// get nude number
    fn size(&mut self) -> usize;
    /// send message
    fn send(&mut self, id: usize, message: &[u8]) -> std::io::Result<usize>;
    /// recv message
    fn recv(&mut self, message: &mut [u8]) -> std::io::Result<usize>;
}

pub struct LoopbackTransport {
    id: usize,
    size: usize,
    base: u16,
    backlog: Arc<Mutex<VecDeque<Vec<u8>>>>,
    _handle: std::thread::JoinHandle<()>,
}

impl LoopbackTransport {
    pub fn new(id: usize, size: usize, base: u16) -> std::io::Result<Self> {
        let backlog = Arc::new(Mutex::new(VecDeque::new()));
        let listener = std::net::TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(127, 0, 0, 1),
            base + id as u16,
        )))?;
        let handle = {
            let backlog = backlog.clone();
            std::thread::spawn(move || {
                for stream in listener.incoming() {
                    let mut stream = stream.unwrap();
                    let mut size = [0u8; 8];
                    stream.read_exact(&mut size).unwrap();
                    let mut buf = vec![0u8; usize::from_be_bytes(size)];
                    stream.read_exact(&mut buf).unwrap();
                    backlog.lock().unwrap().push_back(buf);
                }
            })
        };
        Ok(Self {
            id,
            size,
            base,
            backlog,
            _handle: handle,
        })
    }
}

impl Transport for LoopbackTransport {
    fn id(&mut self) -> usize {
        self.id
    }
    fn size(&mut self) -> usize {
        self.size
    }
    fn send(&mut self, id: usize, message: &[u8]) -> std::io::Result<usize> {
        let mut stream = std::net::TcpStream::connect(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(127, 0, 0, 1),
            self.base + id as u16,
        )))?;
        stream.write(&message.len().to_be_bytes())?;
        stream.write(message)
    }
    fn recv(&mut self, mut message: &mut [u8]) -> std::io::Result<usize> {
        if let Some(msg) = self.backlog.lock().unwrap().pop_front() {
            message.write(&msg)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "no message available",
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::transport::{LoopbackTransport, Transport};
    use std::time::Duration;
    #[test]
    fn send() {
        let message = "hello".as_bytes();
        let mut tp1 = LoopbackTransport::new(0, 2, 3000).unwrap();
        let mut tp2 = LoopbackTransport::new(1, 2, 3000).unwrap();
        tp1.send(1, message).unwrap();
        std::thread::sleep(Duration::from_secs(1));
        let mut buf = vec![0u8; 4096];
        let n = tp2.recv(&mut buf).unwrap();
        assert_eq!(message, &buf[..n]);
    }
}
