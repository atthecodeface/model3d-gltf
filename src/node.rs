// A GLTF node is effectively a 'thing' in the hierarchy of 'thing's in the
// scenes of the gltf
//
// A GLTF node can be a camera; this is the placement and orientation
// of a reference to one of the GltfCamera of the file. A camera should
// not have children.
//
// A GLTF node can be a skin, which is an index into the GltfSkin of the file.
// That skin is only permitted to reference nodes (through its 'skeleton' and
// 'joints' fields) which are referenced by the same scenes as this node is
// (through its hierarchy) referenced from. If this
//
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde_json")]
use serde_json::Value as JsonValue;
#[cfg(not(feature = "serde_json"))]
pub type JsonValue = ();

use mod3d_base::Transformation;

use crate::{CameraIndex, MeshIndex, Named, NodeIndex, SkinIndex};
use crate::{Error, Result};

//a GltfNode
//tp GltfNode
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GltfNode {
    #[cfg_attr(feature = "serde", serde(default))]
    name: String,
    /// The children of the node; if there are none then this is a root node
    children: Vec<NodeIndex>,
    camera: Option<CameraIndex>,
    skin: Option<SkinIndex>,
    matrix: Option<[f32; 16]>,
    mesh: Option<MeshIndex>,
    rotation: Option<[f32; 4]>,
    translation: Option<[f32; 3]>,
    scale: Option<[f32; 3]>,
    weights: Option<JsonValue>,
    #[cfg_attr(feature = "serde", serde(skip))]
    local_transformation: Transformation,
    #[cfg_attr(feature = "serde", serde(skip))]
    global_transformation: Transformation,
    // optional: extensions, extras
}

//ip Named for GltfNode
impl Named for GltfNode {
    type Index = NodeIndex;
    fn is_name(&self, name: &str) -> bool {
        self.name == name
    }
}

//ip GltfNode
impl GltfNode {
    pub fn validate(&self, n: NodeIndex) -> Result<()> {
        if self.skin.is_some() && self.mesh.is_none() {
            return Err(Error::BadJson(format!(
                "Node {n} has a skin but no mesh which is illegal",
            )));
        }
        if self.matrix.is_some()
            && (self.rotation.is_some() || self.translation.is_some() || self.scale.is_some())
        {
            return Err(Error::BadJson(format!(
                "Node {n} has a matrix and some TRS",
            )));
        }

        if self.weights.is_some() {
            return Err(Error::BadJson(format!(
                "Node {n} has morpht target weights that are not supported",
            )));
        }
        Ok(())
    }

    pub fn derive(&mut self, parent_transformation: &Transformation) -> &Transformation {
        self.local_transformation = Transformation::default();
        if let Some(matrix) = self.matrix {
            self.local_transformation.from_mat4(matrix);
        } else {
            if let Some(scale) = self.scale {
                self.local_transformation.set_scale(scale);
            }
            if let Some(rotation) = self.rotation {
                let rotation = (rotation[3], rotation[0], rotation[1], rotation[2]).into();
                self.local_transformation.set_rotation(rotation);
            }
            if let Some(translation) = self.translation {
                self.local_transformation.set_translation(translation);
            }
        }
        self.global_transformation
            .combine(parent_transformation, &self.local_transformation);
        &self.global_transformation
    }

    pub fn is_root(&self) -> bool {
        self.children.is_empty()
    }
    pub fn iter_children(&self) -> std::slice::Iter<NodeIndex> {
        self.children.iter()
    }
    pub fn mesh(&self) -> Option<MeshIndex> {
        self.mesh
    }
    pub fn skin(&self) -> Option<SkinIndex> {
        self.skin
    }
    pub fn camera(&self) -> Option<CameraIndex> {
        self.camera
    }
    pub fn global_transformation(&self) -> &Transformation {
        &self.global_transformation
    }
    pub fn set_mesh(&mut self, mesh: MeshIndex) {
        self.mesh = Some(mesh);
    }
    pub fn set_transformation(&mut self, transformation: &mod3d_base::Transformation) {
        self.local_transformation = *transformation;
    }
    pub fn trans_mut(&mut self) -> &mut mod3d_base::Transformation {
        &mut self.local_transformation
    }
    pub fn derive_gltf(&mut self) {
        if self.local_transformation.scale() != [1., 1., 1.] {
            self.scale = Some(self.local_transformation.scale());
        } else {
            self.scale = None;
        }
        if self.local_transformation.translation() != [0., 0., 0.] {
            self.translation = Some(self.local_transformation.translation());
        } else {
            self.translation = None;
        }
        let (r, i, j, k) = geo_nd::quat::as_rijk(&self.local_transformation.rotation());
        if r != 1.0 {
            self.rotation = Some([i, j, k, r]);
        } else {
            self.rotation = None;
        }
    }
}
