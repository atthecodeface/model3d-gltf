//a Imports
use base64::engine::general_purpose as base64_decoder;
use base64::Engine;

use crate::{Error, Result};

//a Buffer parsing functions
//fp try_buf_parse_base64
/// Attempt to parse a URI as a data::base64 octet stream
///
/// If it is not such a URI then return Ok(None); if it is, then parse
/// it and return Ok(Some(Vec u8))) or Err()
pub fn try_buf_parse_base64(uri: &str, byte_length: usize) -> Result<Option<Vec<u8>>> {
    let Some(data) = uri.strip_prefix("data:application/octet-stream;base64,") else {
        return Ok(None);
    };
    let bytes = base64_decoder::STANDARD.decode(data)?;
    if bytes.len() < byte_length {
        Err(Error::BufferTooShort)
    } else {
        Ok(Some(bytes))
    }
}

//fp buf_parse_fail
/// Return a result that indicates a failure to parse the URI
pub fn buf_parse_fail<T>(_uri: &str, _byte_length: usize) -> Result<T> {
    Err(Error::BufferRead)
}
