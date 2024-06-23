//a Imports
use serde::Deserialize;

use crate::Named;
use crate::{NodeIndex, SceneIndex};

//a GltfScene
//tp GltfScene
/// A type that contains the data from a Gltf Json 'Scene'
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GltfScene {
    /// Optional name of the scene
    pub name: String,
    /// List of nodes in the scene (should not be empty)
    ///
    /// This can include cameras, lights, etc; each must be a root node id
    pub nodes: Vec<NodeIndex>,
}

//ip Named for GltfScene
impl Named for GltfScene {
    type Index = SceneIndex;
    fn is_name(&self, name: &str) -> bool {
        self.name == name
    }
}
