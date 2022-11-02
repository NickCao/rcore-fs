use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    usize,
};

pub trait Transport {
    /// get node id
    fn id(&mut self) -> usize;
    /// get nude number
    fn size(&mut self) -> usize;
    /// exchange message
    fn exchange(
        &mut self,
        id: usize,
        request: &[u8],
        response: &mut [u8],
    ) -> std::io::Result<usize>;
}

pub struct LoopbackTransport {
    id: usize,
    size: usize,
    base: u16,
    _handle: std::thread::JoinHandle<()>,
}

impl LoopbackTransport {
    pub fn new(
        id: usize,
        size: usize,
        base: u16,
        mut callback: Box<dyn FnMut(&[u8], &mut [u8]) -> std::io::Result<usize> + Send>,
    ) -> std::io::Result<Self> {
        let listener = std::net::TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(127, 0, 0, 1),
            base + id as u16,
        )))?;
        let handle = {
            std::thread::spawn(move || {
                for stream in listener.incoming() {
                    let mut stream = stream.unwrap();

                    let mut size = 0usize.to_be_bytes();
                    stream.read_exact(&mut size).unwrap();
                    let mut request = vec![0u8; usize::from_be_bytes(size)];
                    stream.read_exact(&mut request).unwrap();

                    let mut response = vec![0u8; 4096];
                    let n = callback(&request, &mut response).unwrap();

                    stream.write_all(&n.to_be_bytes()).unwrap();
                    stream.write_all(&response[..n]).unwrap();
                }
            })
        };
        Ok(Self {
            id,
            size,
            base,
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
    fn exchange(
        &mut self,
        id: usize,
        request: &[u8],
        response: &mut [u8],
    ) -> std::io::Result<usize> {
        let mut stream = std::net::TcpStream::connect(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(127, 0, 0, 1),
            self.base + id as u16,
        )))?;
        // | size | request |
        stream.write_all(&request.len().to_be_bytes())?;
        stream.write_all(request)?;
        // | size | response |
        let mut size = 0usize.to_be_bytes();
        stream.read_exact(&mut size)?;
        let size = usize::from_be_bytes(size);
        if size > response.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "response buffer too small",
            ));
        }
        stream.read_exact(&mut response[..size])?;
        Ok(size)
    }
}

#[cfg(test)]
mod test {
    use crate::transport::{LoopbackTransport, Transport};
    #[test]
    fn send() {
        static MESSAGE: &[u8] = "hello".as_bytes();
        let mut tp1 = LoopbackTransport::new(0, 2, 3000, Box::new(|_, _| unreachable!())).unwrap();
        LoopbackTransport::new(
            1,
            2,
            3000,
            Box::new(|request, response| {
                response[..request.len()].copy_from_slice(request);
                Ok(request.len())
            }),
        )
        .unwrap();
        let mut buf = [0u8; 4096];
        let n = tp1.exchange(1, MESSAGE, &mut buf).unwrap();
        assert_eq!(MESSAGE, &buf[..n]);
    }
}
