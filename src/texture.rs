use serde;
use serde::Deserialize;
use serde_json::Value as JsonValue;

use crate::Named;
use crate::{ImageIndex, TextureIndex};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GltfTexture {
    /// Optional name of the texture
    pub name: String,
    /// Image index (source)
    #[serde(rename = "source")]
    pub image: ImageIndex,
    /// Sampler
    #[serde(rename = "sampler")]
    pub sampler: JsonValue,
}

impl Named for GltfTexture {
    type Index = TextureIndex;
    fn is_name(&self, name: &str) -> bool {
        self.name == name
    }
}
