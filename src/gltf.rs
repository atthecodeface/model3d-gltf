//a Imports
use model3d_base::hierarchy::Hierarchy;
use model3d_base::Transformation;
use serde_json::Value as JsonValue;

use crate::{AccessorIndex, BufferIndex, MeshIndex, NodeIndex, ViewIndex};
use crate::{GltfAccessor, GltfBuffer, GltfBufferView, GltfJsonValue, GltfMesh, GltfNode};
use crate::{Named, Result};

//a Gltf
//tp Gltf
/// A Gltf file
#[derive(Debug)]
pub struct Gltf {
    /// The parsed JSON value
    json_value: GltfJsonValue,
    /// The hierarchy of nodes
    node_hierarchy: Hierarchy<NodeIndex>,
    /// Which node hierarchy element a particular NodeIndex corresponds to
    nh_index: Vec<usize>,
}

impl Gltf {
    //ap nh_index
    pub fn nh_index(&self, node: NodeIndex) -> usize {
        self.nh_index[node.as_usize()]
    }

    //cp of_json_value
    /// Create a [GltfJsonValue] from a [serde::json::Value], doing
    /// some validation
    pub fn of_json_value(json_value: JsonValue) -> Result<Self> {
        let json_value: GltfJsonValue = serde_json::from_value(json_value)?;
        json_value.validate()?;
        let node_hierarchy = Default::default();
        let nh_index = vec![];
        let mut s = Self {
            json_value,
            node_hierarchy,
            nh_index,
        };
        s.gen_node_hierarchy();
        s.derive();
        Ok(s)
    }

    //mp gen_node_hierarchy
    // Create nodes (componentts) and objects (somehow)
    pub fn gen_node_hierarchy(&mut self) {
        if self.node_hierarchy.len() > 0 {
            return;
        }
        let nodes = self.json_value.nodes();
        self.nh_index = vec![0; nodes.len()];
        for i in 0..nodes.len() {
            self.nh_index[i] = self.node_hierarchy.add_node(i.into());
        }
        for (i, n) in nodes.iter().enumerate() {
            for c in n.iter_children() {
                self.node_hierarchy.relate(i, c.as_usize());
            }
        }
        self.node_hierarchy.find_roots();
    }

    //mp derive
    /// Derive any extra state required
    pub fn derive(&mut self) {
        for r in self.node_hierarchy.borrow_roots() {
            let mut stack = vec![Transformation::default()];
            for x in self.node_hierarchy.enum_from(*r) {
                let (is_push, n, has_children) = x.unpack();
                if is_push {
                    let t = self
                        .json_value
                        .node_derive((*n).into(), stack.last().unwrap());
                    if has_children {
                        stack.push(*t);
                    }
                } else {
                    if has_children {
                        stack.pop();
                    }
                }
            }
        }
    }

    //mp take_buffer_datta
    pub fn take_buffer_data(&mut self, buffer: BufferIndex) -> GltfBuffer {
        self.json_value.take_buffer_data(buffer)
    }

    //ap buffers
    /// Get a reference to the buffers BEFORE THEY HAVE BEEN TAKEN
    pub fn buffers(&self) -> &[GltfBuffer] {
        self.json_value.buffers()
    }

    //ap meshes
    /// Get a reference to the meshes
    pub fn meshes(&self) -> &[GltfMesh] {
        self.json_value.meshes()
    }

    //ap accessors
    /// Get a reference to the accessors
    pub fn accessors(&self) -> &[GltfAccessor] {
        self.json_value.buffer_accessors()
    }

    //ap node_hierarchy
    /// Get a reference to the [Hierarchy] of nodes, as indices
    pub fn node_hierarchy(&self) -> &Hierarchy<NodeIndex> {
        &self.node_hierarchy
    }

    //ap get_node
    /// Get the [NodeIndex] of a named node, a a node by its usize index (as a
    /// string)
    ///
    /// If the node is not found then return None
    pub fn get_node(&self, name: &str) -> Option<NodeIndex> {
        GltfNode::get_named(self.json_value.nodes(), name)
    }
}

//ip Index<NodeIndex> for Gltf
impl std::ops::Index<NodeIndex> for Gltf {
    type Output = GltfNode;
    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.json_value.nodes()[index.as_usize()]
    }
}

//ip Index<AccessorIndex> for Gltf
impl std::ops::Index<AccessorIndex> for Gltf {
    type Output = GltfAccessor;
    fn index(&self, index: AccessorIndex) -> &Self::Output {
        &self.json_value.buffer_accessors()[index.as_usize()]
    }
}

//ip Index<ViewIndex> for Gltf
impl std::ops::Index<ViewIndex> for Gltf {
    type Output = GltfBufferView;
    fn index(&self, index: ViewIndex) -> &Self::Output {
        &self.json_value.buffer_views()[index.as_usize()]
    }
}

//ip Index<MeshIndex> for Gltf
impl std::ops::Index<MeshIndex> for Gltf {
    type Output = GltfMesh;
    fn index(&self, index: MeshIndex) -> &Self::Output {
        &self.json_value.meshes()[index.as_usize()]
    }
}
