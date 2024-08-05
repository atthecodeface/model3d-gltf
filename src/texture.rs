//a Imports
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Named;
use crate::{ImageIndex, SamplerIndex, TextureIndex};

//a GltfTextureInfo
//tp GltfTextureInfo
/// A type representing a Gltf Texture Info, which is instantiated in
/// different ways for different aspects of a material, and which
/// refers to a Texture (and TexCoord number)
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GltfTextureInfo {
    /// Optional name of the texture
    pub index: TextureIndex,
    /// Image index (source)
    #[cfg_attr(feature = "serde", serde(rename = "texCoord"))]
    pub tex_coord: usize,
    /// Scale - for normal textures only
    pub scale: f32,
    /// Strength - for occlusion textures only
    pub strength: f32,
}

impl GltfTextureInfo {
    pub fn index(&self) -> TextureIndex {
        self.index
    }
}

//a GltfTexture
//tp GltfTexture
/// A type representing a Gltf Texture -
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GltfTexture {
    /// Optional name of the texture
    pub name: String,
    /// Image index (source)
    #[cfg_attr(feature = "serde", serde(rename = "source"))]
    pub image: ImageIndex,
    /// Sampler index
    pub sampler: SamplerIndex,
}

impl GltfTexture {
    pub fn image(&self) -> ImageIndex {
        self.image
    }
    pub fn sampler(&self) -> SamplerIndex {
        self.sampler
    }
}

//ip Named for GltfTexture
impl Named for GltfTexture {
    type Index = TextureIndex;
    fn is_name(&self, name: &str) -> bool {
        self.name == name
    }
}
