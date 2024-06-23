//a Imports
use serde::{Deserialize, Deserializer};

use crate::{BufferIndex, ViewIndex};

//a Useful functions
//fi comp_type_to_ele_type
/// Map a Gltf JSON accessor element type integer to a BufferElementType - such
/// as Float32
///
/// If the value is invalid
fn comp_type_to_ele_type<'de, D>(
    de: D,
) -> std::result::Result<model3d_base::BufferElementType, D::Error>
where
    D: Deserializer<'de>,
{
    let c: usize = serde::de::Deserialize::deserialize(de)?;
    use model3d_base::BufferElementType::*;
    Ok(match c {
        5120 => Int8,
        5121 => Int8, // unsigned
        5122 => Int16,
        5123 => Int16, // unsigned
        5124 => Int32,
        5125 => Int32, // unsigned
        5126 => Float32,
        _ => {
            return Err(serde::de::Error::custom(format!(
                "Unknown accessor element type {c}"
            )))
        }
    })
}

//fi ele_type_s32
/// The default element type if not provided bya Gltf JSON
fn ele_type_s32() -> model3d_base::BufferElementType {
    model3d_base::BufferElementType::Int32
}

//fi type_to_num
/// Map a Gltf JSON element array/scalar type name to a number of elements
fn type_to_num<'de, D>(de: D) -> std::result::Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(de)?;
    match s.as_ref() {
        "SCALAR" => Ok(1),
        "VEC2" => Ok(2),
        "VEC3" => Ok(3),
        "VEC4" => Ok(4),
        "MAT2" => Ok(2),
        "MAT3" => Ok(9),
        "MAT4" => Ok(16),
        _ => Err(format!("Unknown accessor type {s}")),
    }
    .map_err(serde::de::Error::custom)
}

//a GltfBuffer
//tp GltfBuffer
///
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GltfBuffer {
    /// The URI specified by the buffer; this might be a data:URI containing
    /// the data itself, or maybe a relative path to a binary data or image
    uri: String,
    /// The byte length of the buffer - any provided URI contents must be at
    /// least this length
    #[serde(rename = "byteLength")]
    byte_length: usize,
}

//ip GltfBuffer
impl GltfBuffer {
    //ap uri
    /// Get a reference to the URI of the buffer
    pub fn uri(&self) -> &str {
        &self.uri
    }

    //ap byte_length
    /// Get the byte length of the buffer
    pub fn byte_length(&self) -> usize {
        self.byte_length
    }

    //mp take_buffer
    /// Take all the contents of the buffer, leaving a buffer in place with an
    /// empty URI
    ///
    /// This allows for a large data: URI to be dropped from the Gltf JSON when
    /// the data has been moved into a real buffer
    pub fn take_buffer(&mut self) -> Self {
        Self {
            uri: std::mem::take(&mut self.uri),
            byte_length: self.byte_length,
        }
    }
}

//tp GltfBufferView
/// A view onto a buffer (refered to be index into the Gltf file array of
/// buffers), referencing a subset of the buffer given by an offset and length
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GltfBufferView {
    buffer: BufferIndex,
    #[serde(rename = "byteLength")]
    byte_length: usize,
    #[serde(rename = "byteOffset")]
    byte_offset: usize,
    #[serde(rename = "byteStride")]
    byte_stride: Option<usize>,
}

impl GltfBufferView {
    //ap buffer
    pub fn buffer(&self) -> BufferIndex {
        self.buffer
    }

    //ap byte_offset
    pub fn byte_offset(&self) -> usize {
        self.byte_offset
    }

    //ap byte_length
    pub fn byte_length(&self) -> usize {
        self.byte_length
    }

    //ap byte_stride
    pub fn byte_stride(&self, default: usize) -> usize {
        self.byte_stride.unwrap_or(default)
    }

    //ap byte_end
    pub fn byte_end(&self) -> usize {
        self.byte_offset + self.byte_length
    }
}

//tp GltfAccessor
/// A Gltf accessor which references a buffer view to provide the data for
/// either indices or an atttribute for a vertex
///
/// An Accessor is a stride-separated set of N-element data structures,
/// providing a list of matrices, vectors, or just scalar sets of floats or ints
///
/// The stride is provided by the buffer view itself, as it is common for all
/// accessors using a buffer view (in Gltf). If the buffer view has a stride of
/// 0 then the actual stride is the size of the N-element type.
#[derive(Debug, Deserialize)]
pub struct GltfAccessor {
    /// The buffer view that contains the data for the accessor
    ///
    /// If this is None then zeros are supposed to be used for the accessor
    /// contents
    #[serde(rename = "bufferView")]
    buffer_view: Option<ViewIndex>,
    /// Byte offset from start of the view (or offset+k*stride) for the
    /// N-element data structure the accessor defines
    #[serde(rename = "byteOffset")]
    #[serde(default)]
    byte_offset: usize,
    /// The type of the element; in Gltf JSON this is encoded with a magic
    /// number; the default value is signed 32-bit integer
    #[serde(rename = "componentType")]
    // 5120-5126: s8, u8, s16, u16, s32, u32, f32 else s32
    #[serde(deserialize_with = "comp_type_to_ele_type")]
    #[serde(default = "ele_type_s32")]
    component_type: model3d_base::BufferElementType,
    #[serde(rename = "count")]
    // minimum 1
    count: usize,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "type_to_num")]
    // SCALAR, VEC2, VEC3, VEC5, MAT2, MAT3, MAT4, string
    elements_per_data: usize,
    // optional: normalized, max, min, sparse
    // optional: name, extensions, extras
}

impl GltfAccessor {
    //ap buffer_view
    pub fn buffer_view(&self) -> Option<ViewIndex> {
        self.buffer_view
    }

    //ap byte_offset
    pub fn byte_offset(&self) -> usize {
        self.byte_offset
    }

    //ap count
    pub fn count(&self) -> usize {
        self.count
    }

    //ap component_type
    pub fn component_type(&self) -> model3d_base::BufferElementType {
        self.component_type
    }

    //ap ele_byte_size
    pub fn ele_byte_size(&self) -> usize {
        self.elements_per_data * self.component_type().byte_length()
    }

    //ap elements_per_data
    pub fn elements_per_data(&self) -> usize {
        self.elements_per_data
    }

    //ap byte_stride
    pub fn byte_stride(&self, view_byte_stride: usize) -> usize {
        if view_byte_stride != 0 {
            view_byte_stride
        } else {
            self.ele_byte_size()
        }
    }

    //ap byte_view_end
    /// Return the byte 1 past the last view byte used
    pub fn byte_view_end(&self, view_byte_stride: usize) -> usize {
        let byte_stride = self.byte_stride(view_byte_stride);
        self.byte_offset + byte_stride * (self.count - 1) + self.ele_byte_size()
    }
}
