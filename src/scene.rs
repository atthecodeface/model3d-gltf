use serde;
use serde::Deserialize;

use crate::Named;
use crate::{NodeIndex, SceneIndex};

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
impl Named for GltfScene {
    type Index = SceneIndex;
    fn is_name(&self, name: &str) -> bool {
        self.name == name
    }
}
