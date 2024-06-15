use serde;
use serde::{Deserialize, Deserializer};

fn comp_type_to_ele_type<'de, D>(
    de: D,
) -> std::result::Result<Option<model3d_base::BufferElementType>, D::Error>
where
    D: Deserializer<'de>,
{
    let c: usize = serde::de::Deserialize::deserialize(de)?;
    use model3d_base::BufferElementType::*;
    Ok(match c {
        5120 => Some(Int8),
        5121 => Some(Int8), // unsigned
        5122 => Some(Int16),
        5123 => Some(Int16), // unsigned
        5124 => Some(Int32),
        5125 => Some(Int32), // unsigned
        5126 => Some(Float32),
        _ => {
            return Err(serde::de::Error::custom(format!(
                "Unknown accessor ele type {c}"
            )))
        }
    })
}

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
        _ => Err(format!("Unknown accessor type {s}")).into(),
    }
    .map_err(serde::de::Error::custom)
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct GltfBuffer {
    uri: String,
    #[serde(rename = "byteLength")]
    byte_length: usize,
}

impl GltfBuffer {
    //ap uri
    pub fn uri(&self) -> &str {
        &self.uri
    }
    //ap byte_length
    pub fn byte_length(&self) -> usize {
        self.byte_length
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct GltfBufferView {
    buffer: usize,
    #[serde(rename = "byteLength")]
    byte_length: usize,
    #[serde(rename = "byteOffset")]
    byte_offset: usize,
    #[serde(rename = "byteStride")]
    byte_stride: Option<usize>,
}

impl GltfBufferView {
    //ap buffer
    pub fn buffer(&self) -> usize {
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
    pub fn byte_stride(&self) -> Option<usize> {
        self.byte_stride
    }

    //ap byte_end
    pub fn byte_end(&self) -> usize {
        self.byte_offset + self.byte_length
    }
}

#[derive(Deserialize)]
pub struct GltfAccessor {
    // Can be left out in which case the data is all zeros
    // This is not supported here
    #[serde(rename = "bufferView")]
    buffer_view: Option<usize>,
    #[serde(rename = "byteOffset")]
    #[serde(default)]
    byte_offset: usize,
    #[serde(rename = "componentType")]
    // 5120-5126: s8, u8, s16, u16, s32, u32, f32 else s32
    #[serde(deserialize_with = "comp_type_to_ele_type")]
    component_type: Option<model3d_base::BufferElementType>,
    #[serde(rename = "count")]
    // minimum 1
    count: usize,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "type_to_num")]
    // SCALAR, VEC2, VEC3, VEC5, MAT2, MAT3, MAT4, string
    num_comp: usize,
    // optional: normalized, max, min, sparse
    // optional: name, extensions, extras
}

impl GltfAccessor {
    //ap buffer_view
    pub fn buffer_view(&self) -> Option<usize> {
        self.buffer_view
    }

    //ap byte_offset
    pub fn byte_offset(&self) -> usize {
        self.byte_offset
    }

    //ap component_type
    pub fn component_type(&self) -> model3d_base::BufferElementType {
        self.component_type
            .unwrap_or(model3d_base::BufferElementType::Int32)
    }

    //ap ele_byte_size
    pub fn ele_byte_size(&self) -> usize {
        self.count * self.component_type().byte_length()
    }

    //ap num_comp
    pub fn num_comp(&self) -> usize {
        self.num_comp
    }
}
