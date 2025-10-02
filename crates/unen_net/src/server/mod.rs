use std::net::{SocketAddr, UdpSocket};

use thiserror::Error;

use crate::socket::{self, socket_poll_from, socket_send_to};

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to bind socket: {0}")]
    SocketBindFailed(#[from] std::io::Error),
    #[error("failed to send data: {0}")]
    SendFailed(String),
    #[error("socket error: {0}")]
    SocketError(#[from] socket::Error),
}

pub fn create_server() -> StoppedServer {
    StoppedServer::default()
}

#[derive(Default)]
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
        Ok(socket_send_to(&self.socket, buf, addr)?)
    }

    pub fn poll(&self) -> Vec<(SocketAddr, Vec<u8>)> {
        socket_poll_from(&self.socket)
    }

    pub fn addr(&self) -> SocketAddr {
        self.socket.local_addr().unwrap()
    }
}
