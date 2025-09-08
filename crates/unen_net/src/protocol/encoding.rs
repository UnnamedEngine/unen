use bincode::{Decode, Encode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("encode error: {0}")]
    EncodeError(#[from] bincode::error::EncodeError),
    #[error("decode error: {0}")]
    DecodeError(#[from] bincode::error::DecodeError),
}

pub fn encode_to_vec<T: Encode>(val: &T) -> Result<Vec<u8>, Error> {
    Ok(bincode::encode_to_vec(val, bincode::config::standard())?)
}

pub fn decode_from_vec<T: Decode<()>>(src: &[u8]) -> Result<T, Error> {
    Ok(bincode::decode_from_slice(src, bincode::config::standard())?.0)
}
