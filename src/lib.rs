/// A GLB file contains a 12-byte header; a chunk 0 (JSON); an optional
/// chunk 1 (binary data)
///
/// The 12-byte header is 0x46546C67_u32 ; 0x00000002_u32; byte_length_u32
///
/// byte_length_u32 must equal the byte size of the GLB file content
///
/// Chunk 0 has an 8-byte header that is byte_length_u32; 0x4E4F534A_u32,
/// followed by data
///
/// byte_length_u32 must equal the byte size of the data - must be a
/// multiple of 4 bytes
///
/// Chunk 1 (optional) has an 8-byte header that is byte_length_u32;
/// 0x004E4942_32, followed by data
///
/// byte_length_u32 must equal the byte size of the data - must be a
/// multiple of 4 bytes
///
///
/// A function is required that takes a GLB file and returns a Gltf Json
/// Value and invokes a callback on the chunk 1 file data to get an
/// Option<Vec<u8>> of chunk 1
///
/// A function is required that takes a Json file and returns a Gltf Json
/// Value
///
/// A method on the Gltf Json Value is required that takes invokes
/// callbacks for each buffer data in the file
///
/// A method on the Gltf Json Value is required that turns it into a Gltf
/// descriptor
///
/// Methods are required on the Gltf descriptor that access the
/// scenes; the default scene; the nodes by name; the skeletons; etc.
///
/// Methods are required that build an Object from a named node
///
/// GLTF notes
///
/// A GLTF file contains zero, one or more Scenes. If it has zero scenes
/// then it is a 'library' of meshes or whatever; this is supported here.
///
/// Each scene consists of an array of *root* nodes (that is, nodes thare
/// are never the children of other nodes). The same node may appear in
/// more than one scene.
///
/// The GLTF file contains an array of nodes, but they actually form a set
/// of node hierarchies. Each hierarchy has a single root and no cycles.
/// Each node is part of a single hierarchy. The hierarchy is indicated by
/// each parent node listing its children nodes by Gltf file node array
/// index.
///
/// A node may have a transformation associated with it; if it does, then
/// everything 'inside' the node uses the a coordinate system that is the
/// transformation applied to its parent coordinate system. The parent
/// coordinate system of a root node is the identity coordinate system.
/// Except when a node is a *skinned mesh* - i.e. it explicitly has a
/// *skin* field and hence also a *mesh* field; then its coordinate system
/// is the identity coordinate system, with the (posed) joints having to
/// provide any mapping of the mesh's coordinates to render coordinates.
///
/// A node *can* be a skinned mesh and a camera and a joint in a skeleton;
/// the spec does not preclude this.
///
/// A Gltf skin defines a collection of nodes as a skeleton; these nodes
/// must have a common ancestor in the node hierarchy (hence they must
/// always be part of the same scene, since each node is part of a single
/// tree and a scene contains trees). Optionally the gltf file specifies
/// some idea of which common ancestor is the 'root' of the skeleton, but
/// this is meaningless if it is not specified to also be a joint.
///
/// A Gltf file allows for a single pose for any one skeleton, as the pose
/// is that specified by the joints that are in the skin. Since a skinned
/// mesh is only positioned (in Gltf) by the positions of the associated
/// joints, a skinned mesh cannot be instantiated into render space at more
/// than one location in a scene.
mod error;
pub use error::{Error, Result};

mod types;
pub use types::*;

mod traits;
pub use traits::Named;

#[cfg(feature = "serde_json")]
mod glb;
#[cfg(feature = "serde_json")]
pub use glb::glb_load;

mod asset;
mod buffer_usage;
mod buffers_accessors;
mod image;
mod material;
mod node;
mod primitives_meshes;
mod scene;
mod texture;

#[cfg(feature = "serde")]
mod deserialize;
#[cfg(feature = "serde")]
mod serialize;

pub use asset::GltfAsset;
pub(crate) use buffer_usage::BufferUsage;
pub use buffers_accessors::{GltfAccessor, GltfBuffer, GltfBufferView};
pub use image::GltfImage;
pub use material::GltfMaterial;
pub use node::GltfNode;
pub use primitives_meshes::{GltfMesh, GltfPrimitive};
pub use scene::GltfScene;
pub use texture::{GltfTexture, GltfTextureInfo};

mod utils;
pub use utils::{buf_parse_fail, try_buf_parse_base64};

mod gltf;
pub use gltf::Gltf;

mod object_data;
pub use object_data::ObjectData;
mod od_use;
pub(crate) use od_use::ODUses;
