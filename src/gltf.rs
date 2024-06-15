use serde;
use serde::{Deserialize, Deserializer};
use serde_json::Value as JsonValue;

use base64::engine::general_purpose as base64_decoder;
use base64::Engine;

use model3d_base::hierarchy::Hierarchy;

use crate::{node, Error, Named, Result};
use crate::{GltfAccessor, GltfBuffer, GltfBufferView, GltfMesh, GltfNode};

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct GltfScene {
    /// Optional name of the scene
    pub name: String,
    /// List of nodes in the scene (should not be empty)
    ///
    /// This can include cameras, lights, etc; each must be a root node id
    pub nodes: Vec<usize>,
}
impl Named for GltfScene {
    fn is_name(&self, name: &str) -> bool {
        self.name == name
    }
}

#[derive(Default, Deserialize)]
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
    scene: Option<usize>,
    materials: JsonValue,
    cameras: JsonValue,
    images: JsonValue,
    samplers: JsonValue,
    textures: JsonValue,
    skins: JsonValue,
    animations: JsonValue,
}

impl GltfJsonValue {
    //mp validate_buffer_views
    /// Validate the contents - check indices in range, etc
    fn validate_buffer_views(&self) -> Result<()> {
        let n = self.buffers.len();
        for bv in &self.buffer_views {
            if bv.buffer() >= n {
                return Err(Error::BadJson(format!(
                    "Buffer view index {0} out of range (must be < {n})",
                    bv.buffer()
                )));
            }
            let l = self.buffers[bv.buffer()].byte_length();
            if bv.byte_end() > l {
                return Err(Error::BadJson(format!(
                    "Buffer view index {0} specifies subrange outside the buffer size {l})",
                    bv.buffer()
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
            if bv_index >= n {
                return Err(Error::BadJson(format!(
                    "Accessor's buffer view index {0} out of range (must be < {n})",
                    bv_index
                )));
            }
            let bv = &self.buffer_views[bv_index];
            let acc_ele_size = acc.ele_byte_size();
            let acc_stride = bv.byte_stride().unwrap_or(acc_ele_size);
            let last_offset =
                acc.byte_offset() + acc_stride * (acc.num_comp() - 1);
            if last_offset + acc_ele_size > bv.byte_length() {
                return Err(Error::BadJson(format!(
                    "Accessor's last element ends beyond end of buffer view index {0}",
                    bv.buffer()
                )));
            }
        }
        Ok(())
    }

    //mp validate
    /// Validate the contents - check indices in range, etc
    fn validate(&self) -> Result<()> {
        self.validate_buffer_views()?;
        self.validate_accessors()?;
        Ok(())
    }

    //mp take_buffers
    pub fn take_buffers(&mut self) -> Vec<GltfBuffer> {
        std::mem::take(&mut self.buffers)
    }
}
pub struct Gltf {
    json_value: GltfJsonValue,
    node_hierarchy: Hierarchy<usize>,
}

impl Gltf {
    //cp of_json_value
    /// Create a [GltfJsonValue] from a [serde::json::Value], doing
    /// some validation
    pub fn of_json_value(json_value: JsonValue) -> Result<Self> {
        let json_value: GltfJsonValue = serde_json::from_value(json_value)?;
        json_value.validate()?;
        let node_hierarchy = Default::default();
        let mut s = Self {
            json_value,
            node_hierarchy,
        };
        s.gen_node_hierarchy();
        Ok(s)
    }

    //mp gen_buffers
    /// Generate a Vec of all the buffers required for the Gltf
    ///
    /// Drop the buffer descriptors in the GltfJsonValue as we go
    ///
    /// The first comes from opt_buffer_0 if Some; this may come from
    /// a Glb file binary chunk, for example.
    ///
    /// The rest are created by invoking buf_parse on the Uri and
    /// byte_length specified in the [GltfJsonValue]
    pub fn gen_buffers<B, BP>(
        &mut self,
        buf_parse: &BP,
        opt_buffer_0: Option<B>,
    ) -> Result<Vec<B>>
    where
        BP: Fn(&str, usize) -> Result<B>,
    {
        let mut result = vec![];
        if let Some(b) = opt_buffer_0 {
            result.push(b);
        }
        let buffers = self.json_value.take_buffers();
        for b in buffers {
            result.push(buf_parse(b.uri(), b.byte_length())?);
        }
        Ok(result)
    }

    //mp gen_byte_buffers
    /// Generate a Vec of all the buffers required for the Gltf
    ///
    /// Drop the buffer descriptors in the GltfJsonValue as we go
    ///
    /// The first comes from opt_buffer_0 if Some; this may come from
    /// a Glb file binary chunk, for example.
    ///
    /// The rest are created by invoking buf_parse on the Uri and
    /// byte_length specified in the [GltfJsonValue]
    pub fn gen_byte_buffers<BP>(
        &mut self,
        buf_parse: &BP,
        opt_buffer_0: Option<Vec<u8>>,
    ) -> Result<Vec<Vec<u8>>>
    where
        BP: Fn(&str, usize) -> Result<Vec<u8>>,
    {
        let mut result = vec![];
        if let Some(b) = opt_buffer_0 {
            result.push(b);
        }
        let buffers = self.json_value.take_buffers();
        for b in buffers {
            if let Some(buf) = try_buf_parse_base64(b.uri(), b.byte_length())? {
                result.push(buf);
                continue;
            }
            result.push(buf_parse(b.uri(), b.byte_length())?);
        }
        Ok(result)
    }

    //mp gen_views_accessors
    // Then create bufferViews and accessors
    // Create the model3d_base for those

    //mp gen_materials
    // Create materials

    //mp gen_meshes
    // Create meshes

    //mp gen_node_hierarchy
    // Create nodes (componentts) and objects (somehow)
    pub fn gen_node_hierarchy(&mut self) {
        if self.node_hierarchy.len() > 0 {
            return;
        }
        for i in 0..self.json_value.nodes.len() {
            self.node_hierarchy.add_node(i);
        }
        for (i, n) in self.json_value.nodes.iter().enumerate() {
            for c in n.iter_children() {
                self.node_hierarchy.relate(i, *c);
            }
        }
        self.node_hierarchy.find_roots();
    }

    pub fn node_hierarchy(&self) -> &Hierarchy<usize> {
        &self.node_hierarchy
    }
    pub fn get_node(&self, name: &str) -> Option<usize> {
        GltfNode::get_named(&self.json_value.nodes, name)
    }
}

pub fn try_buf_parse_base64(
    uri: &str,
    byte_length: usize,
) -> Result<Option<Vec<u8>>> {
    let Some(data) = uri.strip_prefix("data:application/octet-stream;base64,")
    else {
        return Ok(None);
    };
    let bytes = base64_decoder::STANDARD.decode(data)?;
    if bytes.len() < byte_length {
        Err(Error::BufferTooShort)
    } else {
        Ok(Some(bytes))
    }
}

pub fn buf_parse_fail<T>(uri: &str, byte_length: usize) -> Result<T> {
    Err(Error::BufferRead)
}
