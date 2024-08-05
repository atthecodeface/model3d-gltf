//a Imports
use mod3d_base::hierarchy::Hierarchy;
use mod3d_base::Transformation;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde_json")]
use serde_json::Value as JsonValue;

#[cfg(not(feature = "serde_json"))]
pub type JsonValue = ();

use crate::{
    AccessorIndex, BufferIndex, ImageIndex, Indexable, MaterialIndex, MeshIndex, NHIndex,
    NodeIndex, SceneIndex, TextureIndex, ViewIndex,
};
use crate::{Error, Named, Result};
use crate::{
    GltfAccessor, GltfAsset, GltfBuffer, GltfBufferView, GltfImage, GltfMaterial, GltfMesh,
    GltfNode, GltfScene, GltfTexture,
};

//a Gltf
//tp Gltf
/// A Gltf file
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Gltf {
    /// The 'asset' field is required in Gltf; it describes the version of Gltf
    /// that the Json is in, and copyright, generator and other such
    /// information
    asset: GltfAsset,

    /// All the 'buffers' from the Json file, in gltf order; this is
    /// the URI but not any client-side buffer representation
    buffers: Vec<GltfBuffer>,

    /// All the 'bufferViews' from the Json file, in gltf order;
    /// buffers are referred to by BufferIndex into the 'buffers'
    /// property
    #[cfg_attr(feature = "serde", serde(rename = "bufferViews"))]
    buffer_views: Vec<GltfBufferView>,

    /// All the 'accessors' from the Json file, in gltf order;
    /// buffer views are referred to by ViewIndex into the 'buffer_views'
    /// property
    accessors: Vec<GltfAccessor>,

    /// All the 'materials' from the Json file, in gltf order
    /// Currently not filled out
    materials: Vec<GltfMaterial>,

    /// All the 'meshes' from the Json file, in gltf order; the
    /// primitives refer to accessors and materials by AccessorIndex
    /// and MaterialIndex into the relevant properties
    meshes: Vec<GltfMesh>,

    /// All the 'nodes' from the Json file, representing nodes in a
    /// hierarchy of meshes AND nodes in a skeleton - gltf conflates
    /// the two (plus cameras, lights, etc)
    nodes: Vec<GltfNode>,

    /// The default scene to be presented by the gltf
    scene: Option<SceneIndex>,

    /// The scenes in the gltf; each refers to an array of NodeIndex
    /// that are the roots of the (distinct) trees that are to be
    /// rendered for a scene
    scenes: Vec<GltfScene>,

    /// The camers in the gltf; currently unfilled
    cameras: Vec<JsonValue>,

    /// The image descriptors from the Json file; this is the URI or
    /// buffer views, not the underlying image data
    images: Vec<GltfImage>,

    /// The sampler descriptors from the Json file
    samplers: JsonValue,

    /// The texture descriptors from the Json file; these refer to
    /// SamplerIndex and ImageIndex
    textures: Vec<GltfTexture>,

    /// The skin (skeleton) descriptors from the Json file
    skins: Vec<JsonValue>,

    /// The animations in the Json file
    animations: JsonValue,

    /// The hierarchy of nodes
    ///
    /// This is generated after the Json file is read; Gltf requries
    /// the nodes to form distinct trees (each node is in precisely
    /// one tree) so a hierarchy will have each NodeInde once as
    /// either a root or once as the child of a single parent
    #[cfg_attr(feature = "serde", serde(skip_deserializing))]
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    node_hierarchy: Hierarchy<NodeIndex>,

    /// A mapping from NodeIndex to node_hierarchy index
    ///
    /// Which node hierarchy element a particular NodeIndex corresponds to
    #[cfg_attr(feature = "serde", serde(skip_deserializing))]
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    nh_index: Vec<NHIndex>,
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

//ip Index<MaterialIndex> for Gltf
impl std::ops::Index<MaterialIndex> for Gltf {
    type Output = GltfMaterial;
    fn index(&self, index: MaterialIndex) -> &Self::Output {
        &self.materials[index.as_usize()]
    }
}

//ip Gltf
impl Gltf {
    pub fn set_asset(&mut self, asset: GltfAsset) {
        self.asset = asset;
    }

    pub fn add_buffer(&mut self, buffer: GltfBuffer) -> BufferIndex {
        let n = self.buffers.len();
        self.buffers.push(buffer);
        n.into()
    }
    pub fn add_mesh(&mut self, mesh: GltfMesh) -> MeshIndex {
        let n = self.meshes.len();
        self.meshes.push(mesh);
        n.into()
    }
    pub fn add_node(&mut self, node: GltfNode) -> NodeIndex {
        let n = self.nodes.len();
        self.nodes.push(node);
        n.into()
    }
    pub fn add_scene(&mut self, scene: GltfScene) -> SceneIndex {
        let n = self.scenes.len();
        self.scenes.push(scene);
        n.into()
    }
    pub fn add_view(
        &mut self,
        buffer: BufferIndex,
        byte_offset: usize,
        byte_length: usize,
        byte_stride: Option<usize>,
    ) -> ViewIndex {
        let view = GltfBufferView {
            buffer: buffer,
            byte_length: byte_length,
            byte_offset: byte_offset,
            byte_stride: byte_stride,
        }
        .into();

        let n = self.buffer_views.len();
        self.buffer_views.push(view);
        n.into()
    }
    pub fn add_accessor(
        &mut self,
        buffer_view: ViewIndex,
        byte_offset: u32,
        count: u32,
        element_type: mod3d_base::BufferElementType,
        elements_per_data: usize,
    ) -> AccessorIndex {
        let acc = GltfAccessor::new(
            buffer_view,
            byte_offset as usize,
            count as usize,
            element_type,
            elements_per_data,
        );

        let n = self.accessors.len();
        self.accessors.push(acc);
        n.into()
    }
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
                return Err(Error::BadJson(
                    "Accessor is not permitted to not specify a BufferView in this GLTF reader"
                        .into(),
                ));
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
        dbg!(self);
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
    pub fn nh_index(&self, node: NodeIndex) -> NHIndex {
        self.nh_index[node.as_usize()]
    }

    //cp of_json_value
    /// Create a [GltfJsonValue] from a [serde::json::Value], doing
    /// some validation
    #[cfg(feature = "serde_json")]
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
        if !self.node_hierarchy.is_empty() {
            return;
        }
        let n = self.nodes.len();
        self.nh_index = vec![0.into(); n];
        for i in 0..n {
            let ni: NodeIndex = i.into();
            self.nh_index[i] = self.node_hierarchy.add_node(ni).into();
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
                } else if has_children {
                    stack.pop();
                }
            }
        }
    }
}
