//a Imports
use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use crate::{AccessorIndex, MaterialIndex, PrimitiveIndex};

//a Deserializer functions
//fi attr_to_attr
/// Map an array of Gltf string attribute name/value pairs to a Vec of
/// tuples of model3d_base::VertexAttr and AccessorIndex
fn attr_to_attr<'de, D>(
    de: D,
) -> std::result::Result<Vec<(model3d_base::VertexAttr, AccessorIndex)>, D::Error>
where
    D: Deserializer<'de>,
{
    let m: HashMap<String, usize> = serde::de::Deserialize::deserialize(de)?;
    let mut r = vec![];
    for (k, v) in m.into_iter() {
        use model3d_base::VertexAttr::*;
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
/// Map a Gltf primitive type specified by an integer to a model3d_base::PrimitiveType
fn primitive_type<'de, D>(de: D) -> std::result::Result<model3d_base::PrimitiveType, D::Error>
where
    D: Deserializer<'de>,
{
    let p: usize = serde::de::Deserialize::deserialize(de)?;
    use model3d_base::PrimitiveType::*;
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
fn pt_triangles() -> model3d_base::PrimitiveType {
    model3d_base::PrimitiveType::Triangles
}

//a GltfPrimitive
//tp GltfPrimitive
/// A Gltf primitive, as deserialized from the Gltf Json
#[derive(Debug, Deserialize)]
pub struct GltfPrimitive {
    // This must be a map from attribute name to accessor index
    //
    // attribute name - corresponds to model3d_base::VertexAttr
    #[serde(deserialize_with = "attr_to_attr")]
    attributes: Vec<(model3d_base::VertexAttr, AccessorIndex)>,
    // 0-6: POINTS, LINES, LINE_LOOP, LINE_STRIP, TRIANGLES, TRIANGLE_STRIP,
    // TRIANGLE_FAN default is 4:triangles
    #[serde(default = "pt_triangles")]
    #[serde(deserialize_with = "primitive_type")]
    mode: model3d_base::PrimitiveType,
    // optional
    #[serde(default)]
    material: Option<MaterialIndex>,
    // optional - if not present then drawArrays should be used
    #[serde(default)]
    indices: Option<AccessorIndex>,
    // optional: targets
    // optional: extensions, extras
}

//ip GltfPrimitive
impl GltfPrimitive {
    //ap indices
    /// Return the AccessorIndex for the indices of the primitive - or
    /// None if one was not specified (drawArrays should be used to
    /// render the primitive)
    pub fn indices(&self) -> Option<AccessorIndex> {
        self.indices
    }

    //ap primitive_type
    /// Return the model3d_base::PrimitiveType of the primitive
    /// (TriangleStrip, etc)
    pub fn primitive_type(&self) -> model3d_base::PrimitiveType {
        self.mode
    }

    //ap attributes
    /// Return a slice of tuples of model3d_base::VertexAttr and
    /// AccessorIndex from the Gltf for the primitive
    pub fn attributes(&self) -> &[(model3d_base::VertexAttr, AccessorIndex)] {
        &self.attributes
    }

    //ap material
    /// Return the
    /// AccessorIndex from the Gltf for the primitive
    pub fn material(&self) -> Option<MaterialIndex> {
        self.material
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GltfMesh {
    /// The name of the mesh, if any
    #[serde(default)]
    name: String,
    /// The primitives that make up the mesh
    primitives: Vec<GltfPrimitive>,
    // optional: weights (ignored as morph targets are not supported)
    // optional: name, extensions, extras
}

impl GltfMesh {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn primitives(&self) -> &[GltfPrimitive] {
        &self.primitives
    }
}

//ip Index<PrimitiveIndex> for GltfMesh
impl std::ops::Index<PrimitiveIndex> for GltfMesh {
    type Output = GltfPrimitive;
    fn index(&self, index: PrimitiveIndex) -> &Self::Output {
        &self.primitives[index.as_usize()]
    }
}
