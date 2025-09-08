use std::net::{SocketAddr, UdpSocket};

use thiserror::Error;

use crate::protocol::packet::PACKET_MAX_SIZE;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to send data: {0}")]
    SendFailed(String),
}

pub fn socket_send_to(socket: &UdpSocket, buf: &[u8], addr: SocketAddr) -> Result<usize, Error> {
    socket
        .send_to(buf, addr)
        .map_err(|err| Error::SendFailed(err.to_string()))
}

pub fn socket_poll_from(socket: &UdpSocket) -> Vec<(SocketAddr, Vec<u8>)> {
    let mut received = Vec::new();
    let mut buf = [0u8; PACKET_MAX_SIZE];
    while let Ok((len, addr)) = socket.recv_from(&mut buf) {
        received.push((addr, buf[..len].to_vec()));
    }
    received
}

pub fn socket_send(socket: &UdpSocket, buf: &[u8]) -> Result<usize, Error> {
    socket
        .send(buf)
        .map_err(|err| Error::SendFailed(err.to_string()))
}

pub fn socket_poll(socket: &UdpSocket) -> Vec<Vec<u8>> {
    let mut received = Vec::new();
    let mut buf = [0u8; PACKET_MAX_SIZE];
    while let Ok(len) = socket.recv(&mut buf) {
        received.push(buf[..len].to_owned());
    }
    received
}
