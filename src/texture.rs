//a Imports
use serde::Deserialize;

use crate::Named;
use crate::{ImageIndex, SamplerIndex, TextureIndex};

//a GltfTexture
//tp GltfTexture
/// A type representing a Gltf Texture -
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GltfTexture {
    /// Optional name of the texture
    pub name: String,
    /// Image index (source)
    #[serde(rename = "source")]
    pub image: ImageIndex,
    /// Sampler index
    #[serde(rename = "sampler")]
    pub sampler: SamplerIndex,
}

//ip Named for GltfTexture
impl Named for GltfTexture {
    type Index = TextureIndex;
    fn is_name(&self, name: &str) -> bool {
        self.name == name
    }
}
