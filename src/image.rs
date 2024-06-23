//a Imports
use serde::Deserialize;

use crate::Named;
use crate::{ImageIndex, ViewIndex};

//a GltfImage
//tp GltfImage
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GltfImage {
    /// Optional name of the image
    pub name: String,
    /// Optional URI
    pub uri: String,
    /// Optional mime type ("image/jpeg" or "image/png")
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "bufferView")]
    pub buffer_view: ViewIndex,
}

//ip Named for GltfImage
impl Named for GltfImage {
    type Index = ImageIndex;
    fn is_name(&self, name: &str) -> bool {
        self.name == name
    }
}
