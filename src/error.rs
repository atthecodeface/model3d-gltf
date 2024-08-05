use thiserror::Error;

#[derive(Error, Debug)]
pub enum GltfError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[cfg(feature = "serde_json")]
    #[error("Failed to parse GLTF Json")]
    Json(#[from] serde_json::Error),
    #[error("Bad GLTF Json")]
    BadJson(String),
    #[error("Bad UTF8 in GLTF Json")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Bad base64 in GLTF Json")]
    Base64(#[from] base64::DecodeError),
    #[error("Buffer shorter than specified byte_length")]
    BufferTooShort,
    #[error("Buffer could not be read")]
    BufferRead,
    #[error("Failed to load image {reason}")]
    ImageLoad { reason: String },
    #[error("Bad GLB header")]
    GlbHdr,
    #[error("Bad GLB Json header")]
    GlbJsonHdr,
    #[error("IO error reading GLB Json")]
    GlbJsonIo(std::io::Error),
    #[error("GLB json ends beyond file extent")]
    GlbJsonLength,
    #[error("Bad GLB binary buffer header")]
    GlbBinHdr,
    #[error("IO error reading GLB binary")]
    GlbBinIo(std::io::Error),
    #[error("unknown data store error")]
    Unknown,
}

pub type Error = GltfError;
pub type Result<T> = std::result::Result<T, Error>;
