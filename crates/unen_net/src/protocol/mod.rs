use thiserror::Error;

pub mod packet;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Packet error: {0}")]
    PacketError(#[from] packet::Error),
}
