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
        socket_send_to(&self.socket, buf, addr)
    }

    pub fn poll(&self) -> Vec<(SocketAddr, Vec<u8>)> {
        socket_poll(&self.socket)
    }

    pub fn addr(&self) -> SocketAddr {
        self.socket.local_addr().unwrap()
    }
}

fn socket_send_to(socket: &UdpSocket, buf: &[u8], addr: SocketAddr) -> Result<usize, Error> {
    socket.send_to(buf, addr).map_err(|err| Error::SendFailed(err.to_string()))
}

fn socket_poll(socket: &UdpSocket) -> Vec<(SocketAddr, Vec<u8>)> {
    let mut received = Vec::new();
    let mut buf = [0u8; PACKET_MAX_SIZE];
    while let Ok((len, addr)) = socket.recv_from(&mut buf) {
        received.push((addr, buf[..len].to_vec()));
    }
    received
}
