//a Imports
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Named;
use crate::{NodeIndex, SceneIndex};

//a GltfScene
//tp GltfScene
/// A type that contains the data from a Gltf Json 'Scene'
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GltfScene {
    /// Optional name of the scene
    pub name: String,
    /// List of nodes in the scene (should not be empty)
    ///
    /// This can include cameras, lights, etc; each must be a root node id
    pub nodes: Vec<NodeIndex>,
}
impl GltfScene {
    pub fn add_node(&mut self, node: NodeIndex) {
        self.nodes.push(node);
    }
}

//ip Named for GltfScene
impl Named for GltfScene {
    type Index = SceneIndex;
    fn is_name(&self, name: &str) -> bool {
        self.name == name
    }
}
