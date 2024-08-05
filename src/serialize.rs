//a Imports
use serde;
use serde::{Serialize, Serializer};

use crate::AccessorIndex;

//a Useful functions
//fi attr_to_attr
/// Map an array of Gltf string attribute name/value pairs to a Vec of
/// tuples of mod3d_base::VertexAttr and AccessorIndex
pub fn attr_to_attr<S>(
    attr: &[(mod3d_base::VertexAttr, AccessorIndex)],
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use std::collections::HashMap;
    let mut m = HashMap::<String, usize>::default();
    for (k, v) in attr.iter() {
        use mod3d_base::VertexAttr::*;
        let k = {
            match k {
                Position => "POSITION",
                Normal => "NORMAL",
                Color => "COLOR_0",
                Tangent => "TANGENT",
                Joints => "JOINTS_0",
                Weights => "WEIGHTS_0",
                TexCoords0 => "TEXCOORD_0",
                TexCoords1 => "TEXCOORD_1",
                _ => {
                    return Err(serde::ser::Error::custom(format!(
                        "Unknown Gltf attribute {k:?}"
                    )));
                }
            }
        };
        m.insert(k.to_string(), (*v).into());
    }
    m.serialize(ser)
}

//fi primitive_type
/// Map a Gltf primitive type specified by an integer to a mod3d_base::PrimitiveType
pub fn primitive_type<S>(
    primitive_type: &mod3d_base::PrimitiveType,
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use mod3d_base::PrimitiveType::*;
    let c: u32 = {
        #[allow(unreachable_patterns)]
        match *primitive_type {
            Points => 0,
            Lines => 1,
            LineLoop => 2,
            LineStrip => 3,
            Triangles => 4,
            TriangleStrip => 5,
            TriangleFan => 6,
            _ => {
                return Err(serde::ser::Error::custom(format!(
                    "No GLTF element type for {primitive_type:?}"
                )))
            }
        }
    };
    ser.serialize_u32(c)
}

//fi num_to_type
pub fn num_to_type<S>(count: &usize, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let c = {
        match count {
            1 => "SCALAR",
            2 => "VEC2",
            3 => "VEC3",
            4 => "VEC4", // might be MAT2...
            9 => "MAT3",
            16 => "MAT4",
            _ => {
                return Err(serde::ser::Error::custom(format!(
                    "No GLTF scalar/vec/mat type for {count}"
                )))
            }
        }
    };
    c.serialize(ser)
}

//fi ele_type_to_comp_type
/// Map a Gltf JSON accessor element type integer to a BufferElementType - such
/// as Float32
///
/// If the value is invalid
pub fn ele_type_to_comp_type<S>(
    ele_type: &mod3d_base::BufferElementType,
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use mod3d_base::BufferElementType::*;
    let c: u32 = {
        match *ele_type {
            Int8 => 5120,
            // Int8 => 5121, unsigned
            Int16 => 5122,
            // Int16 => 5123, // unsigned
            // Int32 => 5124, // signed
            Int32 => 5125, // unsigned
            Float32 => 5126,
            _ => {
                return Err(serde::ser::Error::custom(format!(
                    "No GLTF element type for {ele_type:?}"
                )))
            }
        }
    };
    c.serialize(ser)
}
