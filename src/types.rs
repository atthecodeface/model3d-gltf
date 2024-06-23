use serde::{self, Deserialize};

macro_rules! index_type {
    ( $t:ident ) => {
        #[derive(Default, Debug, Copy, Clone, Deserialize, PartialEq, Eq)]
        #[serde(transparent)]
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
        impl From<usize> for $t {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
        impl $t {
            pub fn as_usize(self) -> usize {
                self.0
            }
        }
    };
}

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
