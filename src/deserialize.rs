//a Imports
use std::collections::HashMap;

use serde;
use serde::{Deserialize, Deserializer};

use crate::AccessorIndex;

//a Deserializer functions
//fi attr_to_attr
/// Map an array of Gltf string attribute name/value pairs to a Vec of
/// tuples of mod3d_base::VertexAttr and AccessorIndex
pub fn attr_to_attr<'de, D>(
    de: D,
) -> std::result::Result<Vec<(mod3d_base::VertexAttr, AccessorIndex)>, D::Error>
where
    D: Deserializer<'de>,
{
    let m: HashMap<String, usize> = Deserialize::deserialize(de)?;
    let mut r = vec![];
    for (k, v) in m.into_iter() {
        use mod3d_base::VertexAttr::*;
        let k = match k.as_ref() {
            "POSITION" => Position,
            "NORMAL" => Normal,
            "COLOR_0" => Color,
            "TANGENT" => Tangent,
            "JOINTS_0" => Joints,
            "WEIGHTS_0" => Weights,
            "TEXCOORD_0" => TexCoords0,
            "TEXCOORD_1" => TexCoords1,
            _ => {
                return Err(serde::de::Error::custom(format!("Unknown attribute {k}")));
            }
        };
        r.push((k, v.into()));
    }
    Ok(r)
}

//fi primitive_type
/// Map a Gltf primitive type specified by an integer to a mod3d_base::PrimitiveType
pub fn primitive_type<'de, D>(de: D) -> std::result::Result<mod3d_base::PrimitiveType, D::Error>
where
    D: Deserializer<'de>,
{
    let p: usize = Deserialize::deserialize(de)?;
    use mod3d_base::PrimitiveType::*;
    let pt = match p {
        0 => Points,
        1 => Lines,
        2 => LineLoop,
        3 => LineStrip,
        4 => Triangles,
        5 => TriangleStrip,
        6 => TriangleFan,
        _ => {
            return Err(serde::de::Error::custom(format!(
                "Unknown primitive mode {p}"
            )));
        }
    };
    Ok(pt)
}

//fi pt_triangles
/// Return the default type for a Gltf primitive type - Triangles
pub fn pt_triangles() -> mod3d_base::PrimitiveType {
    mod3d_base::PrimitiveType::Triangles
}
//a Useful functions
//fi comp_type_to_ele_type
/// Map a Gltf JSON accessor element type integer to a BufferElementType - such
/// as Float32
///
/// If the value is invalid
pub fn comp_type_to_ele_type<'de, D>(
    de: D,
) -> std::result::Result<mod3d_base::BufferElementType, D::Error>
where
    D: Deserializer<'de>,
{
    let c: usize = Deserialize::deserialize(de)?;
    use mod3d_base::BufferElementType::*;
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
pub fn ele_type_s32() -> mod3d_base::BufferElementType {
    mod3d_base::BufferElementType::Int32
}

//fi type_to_num
/// Map a Gltf JSON element array/scalar type name to a number of elements
pub fn type_to_num<'de, D>(de: D) -> std::result::Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(de)?;
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

pub fn f32_one() -> f32 {
    1.0
}
