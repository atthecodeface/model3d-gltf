//a Imports
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use crate::{deserialize, serialize};

use crate::{AccessorIndex, Indexable, MaterialIndex, PrimitiveIndex};

//a GltfPrimitive
//tp GltfPrimitive
/// A Gltf primitive, as deserialized from the Gltf Json
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GltfPrimitive {
    // This must be a map from attribute name to accessor index
    //
    // attribute name - corresponds to mod3d_base::VertexAttr
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize::attr_to_attr")
    )]
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize::attr_to_attr"))]
    attributes: Vec<(mod3d_base::VertexAttr, AccessorIndex)>,
    // 0-6: POINTS, LINES, LINE_LOOP, LINE_STRIP, TRIANGLES, TRIANGLE_STRIP,
    // TRIANGLE_FAN default is 4:triangles
    #[cfg_attr(feature = "serde", serde(default = "deserialize::pt_triangles"))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize::primitive_type")
    )]
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize::primitive_type"))]
    mode: mod3d_base::PrimitiveType,
    // optional
    #[cfg_attr(feature = "serde", serde(default))]
    material: Option<MaterialIndex>,
    // optional - if not present then drawArrays should be used
    #[cfg_attr(feature = "serde", serde(default))]
    indices: Option<AccessorIndex>,
    // optional: targets
    // optional: extensions, extras
}

//ip GltfPrimitive
impl GltfPrimitive {
    pub fn new(
        mode: mod3d_base::PrimitiveType,
        indices: Option<AccessorIndex>,
        material: Option<MaterialIndex>,
    ) -> Self {
        Self {
            mode,
            indices,
            material,
            ..Default::default()
        }
    }
    //ap indices
    /// Return the AccessorIndex for the indices of the primitive - or
    /// None if one was not specified (drawArrays should be used to
    /// render the primitive)
    pub fn indices(&self) -> Option<AccessorIndex> {
        self.indices
    }

    //ap primitive_type
    /// Return the mod3d_base::PrimitiveType of the primitive
    /// (TriangleStrip, etc)
    pub fn primitive_type(&self) -> mod3d_base::PrimitiveType {
        self.mode
    }

    //ap attributes
    /// Return a slice of tuples of mod3d_base::VertexAttr and
    /// AccessorIndex from the Gltf for the primitive
    pub fn attributes(&self) -> &[(mod3d_base::VertexAttr, AccessorIndex)] {
        &self.attributes
    }

    //ap material
    /// Return the
    /// AccessorIndex from the Gltf for the primitive
    pub fn material(&self) -> Option<MaterialIndex> {
        self.material
    }
    pub fn add_attribute(&mut self, attr: mod3d_base::VertexAttr, accessor: AccessorIndex) {
        self.attributes.push((attr, accessor))
    }
}

//tp GltfMesh
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GltfMesh {
    /// The name of the mesh, if any
    #[cfg_attr(feature = "serde", serde(default))]
    name: String,
    /// The primitives that make up the mesh
    primitives: Vec<GltfPrimitive>,
    // optional: weights (ignored as morph targets are not supported)
    // optional: name, extensions, extras
}

impl GltfMesh {
    pub fn add_primitive(
        &mut self,
        mode: mod3d_base::PrimitiveType,
        indices: Option<AccessorIndex>,
        material: Option<MaterialIndex>,
    ) -> PrimitiveIndex {
        let p = GltfPrimitive::new(mode, indices, material);
        let n = self.primitives.len();
        self.primitives.push(p);
        n.into()
    }
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

//ip IndexMut<PrimitiveIndex> for GltfMesh
impl std::ops::IndexMut<PrimitiveIndex> for GltfMesh {
    fn index_mut(&mut self, index: PrimitiveIndex) -> &mut Self::Output {
        &mut self.primitives[index.as_usize()]
    }
}
