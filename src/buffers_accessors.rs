//a Imports
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use crate::{deserialize, serialize};

use crate::{BufferIndex, ViewIndex};

//a GltfBuffer
//tp GltfBuffer
///
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GltfBuffer {
    /// The URI specified by the buffer; this might be a data:URI containing
    /// the data itself, or maybe a relative path to a binary data or image
    uri: String,
    /// The byte length of the buffer - any provided URI contents must be at
    /// least this length
    #[cfg_attr(feature = "serde", serde(rename = "byteLength"))]
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

    //cp of_base64
    /// Create from a Base64
    pub fn of_base64<T: AsRef<[u8]>>(data: T) -> Self {
        use base64::engine::general_purpose;
        use base64::Engine;
        let byte_length = data.as_ref().len();
        let mut uri = general_purpose::STANDARD.encode(data);
        uri.insert_str(0, "data:application/octet-stream;base64,");
        Self { uri, byte_length }
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
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GltfBufferView {
    pub buffer: BufferIndex,
    #[cfg_attr(feature = "serde", serde(rename = "byteLength"))]
    pub byte_length: usize,
    #[cfg_attr(feature = "serde", serde(rename = "byteOffset"))]
    pub byte_offset: usize,
    #[cfg_attr(feature = "serde", serde(rename = "byteStride"))]
    pub byte_stride: Option<usize>,
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
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(default))]
pub struct GltfAccessor {
    /// The buffer view that contains the data for the accessor
    ///
    /// If this is None then zeros are supposed to be used for the accessor
    /// contents
    #[cfg_attr(feature = "serde", serde(rename = "bufferView"))]
    buffer_view: Option<ViewIndex>,
    /// Byte offset from start of the view (or offset+k*stride) for the
    /// N-element data structure the accessor defines
    #[cfg_attr(feature = "serde", serde(rename = "byteOffset"))]
    #[cfg_attr(feature = "serde", serde(default))]
    byte_offset: usize,
    /// The type of the element; in Gltf JSON this is encoded with a magic
    /// number; the default value is signed 32-bit integer
    #[cfg_attr(feature = "serde", serde(rename = "componentType"))]
    // 5120-5126: s8, u8, s16, u16, s32, u32, f32 else s32
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize::comp_type_to_ele_type")
    )]
    #[cfg_attr(feature = "serde", serde(default = "deserialize::ele_type_s32"))]
    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "serialize::ele_type_to_comp_type")
    )]
    component_type: mod3d_base::BufferElementType,
    #[cfg_attr(feature = "serde", serde(rename = "count"))]
    // minimum 1
    count: usize,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize::type_to_num")
    )]
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize::num_to_type"))]
    // SCALAR, VEC2, VEC3, VEC5, MAT2, MAT3, MAT4, string
    elements_per_data: usize,
    // optional: normalized, max, min, sparse
    // optional: name, extensions, extras
}

impl GltfAccessor {
    pub fn new(
        buffer_view: ViewIndex,
        byte_offset: usize,
        count: usize,
        component_type: mod3d_base::BufferElementType,
        elements_per_data: usize,
    ) -> Self {
        let buffer_view = Some(buffer_view);
        Self {
            buffer_view,
            byte_offset,
            count,
            component_type,
            elements_per_data,
        }
    }

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
    pub fn component_type(&self) -> mod3d_base::BufferElementType {
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
