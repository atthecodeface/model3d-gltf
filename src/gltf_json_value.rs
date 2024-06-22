use model3d_base::Transformation;
use serde;
use serde::Deserialize;
use serde_json::Value as JsonValue;

use crate::{BufferIndex, GltfAccessor, GltfBuffer, GltfBufferView, GltfMesh, GltfNode, GltfScene};
use crate::{Error, Result};
use crate::{NodeIndex, SceneIndex};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GltfJsonValue {
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
    images: JsonValue,
    samplers: JsonValue,
    textures: JsonValue,
    skins: Vec<JsonValue>,
    animations: JsonValue,
}

impl GltfJsonValue {
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

    pub fn node_derive(
        &mut self,
        node: NodeIndex,
        parent_transformation: &Transformation,
    ) -> &Transformation {
        self.nodes[node.as_usize()].derive(parent_transformation)
    }

    //mp buffers
    pub fn buffers(&self) -> &[GltfBuffer] {
        &self.buffers
    }

    //mp take_buffer_data
    pub fn take_buffer_data(&mut self, buffer: BufferIndex) -> GltfBuffer {
        self.buffers[buffer.as_usize()].take_buffer()
    }

    //ap buffer_views
    pub fn buffer_views(&self) -> &[GltfBufferView] {
        &self.buffer_views
    }

    //ap buffer_accessors
    pub fn buffer_accessors(&self) -> &[GltfAccessor] {
        &self.accessors
    }

    //ap nodes
    pub fn nodes(&self) -> &[GltfNode] {
        &self.nodes
    }

    //ap meshs
    pub fn meshes(&self) -> &[GltfMesh] {
        &self.meshes
    }
}
