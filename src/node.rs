// A GLTF node is effectively a 'thing' in the hierarchy of 'thing's in the
// scenes of the gltf
//
// A GLTF node can be a camera; this is the placement and orientation
// of a reference to one of the GltfCamera of the file. A camera should
// not have children.
//
// A GLTF node can be a skin, which is an index into the GltfSkin of the file.
// That skin is only permitted to reference nodes (through its 'skeleton' and
// 'joints' fields) which are referenced by the same scenes as this node is
// (through its hierarchy) referenced from. If this
//
use serde;
use serde::{Deserialize, Deserializer};
use serde_json::Value as JsonValue;

use crate::Named;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct GltfNode {
    #[serde(default)]
    name: String,
    /// The children of the node; if there are none then this is a root node
    children: Vec<usize>,
    camera: Option<usize>,
    skin: Option<usize>,
    matrix: Option<[f32; 16]>,
    mesh: usize,
    rotation: Option<[f32; 4]>,
    translation: Option<[f32; 3]>,
    scale: Option<[f32; 3]>,
    weights: Option<JsonValue>,
    // optional: extensions, extras
}

impl Named for GltfNode {
    fn is_name(&self, name: &str) -> bool {
        self.name == name
    }
}
impl GltfNode {
    pub fn is_root(&self) -> bool {
        self.children.is_empty()
    }
    pub fn iter_children(&self) -> std::slice::Iter<usize> {
        self.children.iter()
    }
}
