//a Imports
use model3d_base::hierarchy::Hierarchy;
use model3d_base::Transformation;
use serde;
use serde::Deserialize;
use serde_json::Value as JsonValue;

use crate::{
    AccessorIndex, BufferIndex, ImageIndex, MeshIndex, NodeIndex, SceneIndex, TextureIndex,
    ViewIndex,
};
use crate::{Error, Named, Result};
use crate::{
    GltfAccessor, GltfBuffer, GltfBufferView, GltfImage, GltfMesh, GltfNode, GltfScene, GltfTexture,
};

//a Gltf
//tp Gltf
/// A Gltf file
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Gltf {
    /// The 'asset' field is required in Gltf; it describes the version of Gltf
    /// that the Json is in, and copyright, generator and other such
    /// information
    asset: JsonValue,
    buffers: Vec<GltfBuffer>,
    #[serde(rename = "bufferViews")]
    buffer_views: Vec<GltfBufferView>,
    accessors: Vec<GltfAccessor>,
    meshes: Vec<GltfMesh>,
    nodes: Vec<GltfNode>,
    scenes: Vec<GltfScene>,
    scene: Option<SceneIndex>,
    materials: Vec<JsonValue>,
    cameras: Vec<JsonValue>,
    images: Vec<GltfImage>,
    samplers: JsonValue,
    textures: Vec<GltfTexture>,
    skins: Vec<JsonValue>,
    animations: JsonValue,

    /// The hierarchy of nodes
    #[serde(skip_deserializing)]
    node_hierarchy: Hierarchy<NodeIndex>,

    /// Which node hierarchy element a particular NodeIndex corresponds to
    #[serde(skip_deserializing)]
    nh_index: Vec<usize>,
}

//ip Index<NodeIndex> for Gltf
impl std::ops::Index<NodeIndex> for Gltf {
    type Output = GltfNode;
    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.nodes[index.as_usize()]
    }
}

//ip Index<AccessorIndex> for Gltf
impl std::ops::Index<AccessorIndex> for Gltf {
    type Output = GltfAccessor;
    fn index(&self, index: AccessorIndex) -> &Self::Output {
        &self.accessors[index.as_usize()]
    }
}

//ip Index<ViewIndex> for Gltf
impl std::ops::Index<ViewIndex> for Gltf {
    type Output = GltfBufferView;
    fn index(&self, index: ViewIndex) -> &Self::Output {
        &self.buffer_views[index.as_usize()]
    }
}

//ip Index<MeshIndex> for Gltf
impl std::ops::Index<MeshIndex> for Gltf {
    type Output = GltfMesh;
    fn index(&self, index: MeshIndex) -> &Self::Output {
        &self.meshes[index.as_usize()]
    }
}

//ip Index<ImageIndex> for Gltf
impl std::ops::Index<ImageIndex> for Gltf {
    type Output = GltfImage;
    fn index(&self, index: ImageIndex) -> &Self::Output {
        &self.images[index.as_usize()]
    }
}

//ip Index<TextureIndex> for Gltf
impl std::ops::Index<TextureIndex> for Gltf {
    type Output = GltfTexture;
    fn index(&self, index: TextureIndex) -> &Self::Output {
        &self.textures[index.as_usize()]
    }
}

//ip Gltf
impl Gltf {
    //mp validate_buffer_views
    /// Validate the contents - check indices in range, etc
    fn validate_buffer_views(&self) -> Result<()> {
        let n = self.buffers.len();
        for (i, bv) in self.buffer_views.iter().enumerate() {
            let b = bv.buffer();
            if b.as_usize() >= n {
                return Err(Error::BadJson(format!(
                    "Buffer view index {i} has buffer {b} out of range (must be < {n})",
                )));
            }
            let l = self.buffers[b.as_usize()].byte_length();
            if bv.byte_end() > l {
                return Err(Error::BadJson(format!(
                    "Buffer view index {i} specifies subrange outside the buffer size {l})",
                )));
            }
        }
        Ok(())
    }

    //mp validate_accessors
    /// Validate the contents - check indices in range, etc
    fn validate_accessors(&self) -> Result<()> {
        let n = self.buffer_views.len();
        for acc in &self.accessors {
            let bv_index = acc.buffer_view();
            let Some(bv_index) = bv_index else {
                return Err(Error::BadJson(format!(
                    "Accessor is not permitted to not specify a BufferView in this GLTF reader"
                )));
            };
            if bv_index.as_usize() >= n {
                return Err(Error::BadJson(format!(
                    "Accessor's buffer view index {0} out of range (must be < {n})",
                    bv_index
                )));
            }
            let bv = &self.buffer_views[bv_index.as_usize()];
            let acc_byte_end = acc.byte_view_end(bv.byte_stride(0));
            if acc_byte_end > bv.byte_length() {
                return Err(Error::BadJson(format!(
                    "Accessor's last element ends (@{0}) beyond end of buffer view index {1} (at {2})",
                    acc_byte_end,
                    bv.buffer(),
                    bv.byte_length()
                )));
            }
        }
        Ok(())
    }

    //mp validate_nodes
    pub fn validate_nodes(&self) -> Result<()> {
        let l = self.nodes.len();
        for (i, n) in self.nodes.iter().enumerate() {
            for c in n.iter_children() {
                if c.as_usize() >= l {
                    return Err(Error::BadJson(format!(
                        "Node {i} has child index {c} out of range",
                    )));
                }
            }
            if let Some(m) = n.mesh() {
                if m.as_usize() > self.meshes.len() {
                    return Err(Error::BadJson(format!(
                        "Node {i} has mesh index {m} out of range",
                    )));
                }
            }
            if let Some(c) = n.camera() {
                if c.as_usize() > self.cameras.len() {
                    return Err(Error::BadJson(format!(
                        "Node {i} has camera index {c} out of range",
                    )));
                }
            }
            if let Some(s) = n.skin() {
                if s.as_usize() > self.skins.len() {
                    return Err(Error::BadJson(format!(
                        "Node {i} has skin index {s} out of range",
                    )));
                }
            }
            n.validate(i.into())?;
        }
        Ok(())
    }

    //mp validate
    /// Validate the contents - check indices in range, etc
    pub fn validate(&self) -> Result<()> {
        self.validate_buffer_views()?;
        self.validate_accessors()?;
        self.validate_nodes()?;
        Ok(())
    }

    //ap buffers
    pub fn buffers(&self) -> &[GltfBuffer] {
        &self.buffers
    }

    //ap accessors
    /// Get a reference to the accessors
    pub fn accessors(&self) -> &[GltfAccessor] {
        &self.accessors
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
        GltfNode::get_named(self.nodes(), name)
    }

    //mp take_buffer_data
    pub fn take_buffer_data(&mut self, buffer: BufferIndex) -> GltfBuffer {
        self.buffers[buffer.as_usize()].take_buffer()
    }

    //ap buffer_views
    pub fn buffer_views(&self) -> &[GltfBufferView] {
        &self.buffer_views
    }

    //ap nodes
    pub fn nodes(&self) -> &[GltfNode] {
        &self.nodes
    }

    //ap meshes
    pub fn meshes(&self) -> &[GltfMesh] {
        &self.meshes
    }

    //ap nh_index
    pub fn nh_index(&self, node: NodeIndex) -> usize {
        self.nh_index[node.as_usize()]
    }

    //cp of_json_value
    /// Create a [GltfJsonValue] from a [serde::json::Value], doing
    /// some validation
    pub fn of_json_value(json_value: JsonValue) -> Result<Self> {
        let mut s: Self = serde_json::from_value(json_value)?;
        s.validate()?;
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
        let n = self.nodes.len();
        self.nh_index = vec![0; n];
        for i in 0..n {
            self.nh_index[i] = self.node_hierarchy.add_node(i.into());
        }
        for (i, n) in self.nodes.iter().enumerate() {
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
                    let t = self.nodes[*n].derive(stack.last().unwrap());
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
}
