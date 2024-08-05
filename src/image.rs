//a Imports
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Named;
use crate::{ImageIndex, ViewIndex};

//a GltfImage
//tp GltfImage
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GltfImage {
    /// Optional name of the image
    pub name: String,
    /// Optional URI
    pub uri: Option<String>,
    /// Optional mime type ("image/jpeg" or "image/png")
    #[cfg_attr(feature = "serde", serde(rename = "mimeType"))]
    pub mime_type: String,
    #[cfg_attr(feature = "serde", serde(rename = "bufferView"))]
    pub buffer_view: ViewIndex,
}

//ip GltfImage
impl GltfImage {
    //ap uri
    pub fn uri(&self) -> Option<&str> {
        self.uri.as_deref()
    }

    //ap buffer_view
    pub fn buffer_view(&self) -> ViewIndex {
        self.buffer_view
    }

    //ap mime_type
    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }
}

//ip Named for GltfImage
impl Named for GltfImage {
    type Index = ImageIndex;
    fn is_name(&self, name: &str) -> bool {
        self.name == name
    }
}
