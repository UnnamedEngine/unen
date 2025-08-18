use std::net::{SocketAddr, UdpSocket};

use thiserror::Error;

use crate::protocol::packet::PACKET_MAX_SIZE;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to bind socket: {0}")]
    SocketBindFailed(#[from] std::io::Error),
    #[error("failed to send data: {0}")]
    SendFailed(String),
}

pub fn create_server() -> StoppedServer {
    StoppedServer {}
}

pub struct StoppedServer {}

impl StoppedServer {
    pub fn listen(self, addr: SocketAddr) -> Result<ListeningServer, Error> {
        let socket = UdpSocket::bind(addr)?;
        socket.set_nonblocking(true)?;

        Ok(ListeningServer { socket })
    }
}

pub struct ListeningServer {
    socket: UdpSocket,
}

impl ListeningServer {
    pub fn stop(self) -> StoppedServer {
        StoppedServer {}
    }

    pub fn send_to(&self, buf: &[u8], addr: SocketAddr) -> Result<usize, Error> {
        match self.socket.send_to(buf, addr) {
            Ok(sent_bytes) => Ok(sent_bytes),
            Err(err) => Err(Error::SendFailed(err.to_string())),
        }
    }

    pub fn poll(&self) -> Vec<(SocketAddr, Vec<u8>)> {
        let mut received_data = Vec::new();
        let mut buf = [0u8; PACKET_MAX_SIZE];
        while let Ok((len, addr)) = self.socket.recv_from(&mut buf) {
            received_data.push((addr, buf[..len].to_vec()));
        }
        received_data
    }

    pub fn addr(&self) -> SocketAddr {
        self.socket.local_addr().unwrap()
    }
}
