#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

//tp Gltf
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GltfAsset {
    copyright: String,
    version: String,
}

impl GltfAsset {
    pub fn new(copyright: String) -> Self {
        let version = "2.0".into();
        Self { copyright, version }
    }
}
