//a Imports
#[cfg(feature = "serde")]
use serde::{self, Deserialize, Serialize};

//a Indexable and index_type macro
//tt Indexable
pub trait Indexable:
    Sized + std::ops::Deref<Target = usize> + std::ops::DerefMut + std::convert::From<usize>
{
    fn as_usize(&self) -> usize;
}

//mi index_type
macro_rules! index_type {
    ( $t:ident ) => {
        #[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        pub struct $t(usize);
        impl std::ops::Deref for $t {
            type Target = usize;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl std::ops::DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl From<$t> for usize {
            fn from(value: $t) -> usize {
                value.0
            }
        }
        impl From<usize> for $t {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }
        impl From<$t> for mod3d_base::ShortIndex {
            fn from(value: $t) -> mod3d_base::ShortIndex {
                value.0.into()
            }
        }
        impl From<&$t> for mod3d_base::ShortIndex {
            fn from(value: &$t) -> mod3d_base::ShortIndex {
                value.0.into()
            }
        }
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
        impl Indexable for $t {
            fn as_usize(&self) -> usize {
                self.0
            }
        }
    };
}

//a Index types
index_type!(NHIndex);

index_type!(MeshIndex);
index_type!(NodeIndex);
index_type!(CameraIndex);
index_type!(SkinIndex);
index_type!(SceneIndex);
index_type!(ViewIndex);
index_type!(BufferIndex);
index_type!(AccessorIndex);
index_type!(ImageIndex);
index_type!(TextureIndex);
index_type!(MaterialIndex);
index_type!(SamplerIndex);
index_type!(PrimitiveIndex);

index_type!(ODBufIndex);
index_type!(ODBufDataIndex);
index_type!(ODAccIndex);
index_type!(ODVerticesIndex);
index_type!(ODImagesIndex);
index_type!(ODTexturesIndex);
index_type!(ODMaterialsIndex);
