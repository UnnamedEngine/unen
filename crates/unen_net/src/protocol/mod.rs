use thiserror::Error;

pub mod encoding;
pub mod packet;

#[derive(Debug, Error)]
pub enum Error {
    #[error("packet error: {0}")]
    PacketError(#[from] packet::Error),
    #[error("encoding error: {0}")]
    EncodingError(#[from] encoding::Error),
}
