//a Imports
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use crate::deserialize;

use crate::Named;
use crate::{GltfTextureInfo, MaterialIndex};

//tp GltfPbrMetallicRoughness
///
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GltfPbrMetallicRoughness {
    /// Base color factor
    #[cfg_attr(feature = "serde", serde(rename = "baseColorFactor"))]
    pub base_color_factor: Option<Vec<f32>>,
    #[cfg_attr(feature = "serde", serde(rename = "baseColorTexture"))]
    pub base_color_texture: Option<GltfTextureInfo>,
    #[cfg_attr(feature = "serde", serde(rename = "metallicRoughnessTexture"))]
    pub metallic_roughness_texture: Option<GltfTextureInfo>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "metallicFactor", default = "deserialize::f32_one")
    )]
    pub metallic_factor: f32,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "roughnessFactor", default = "deserialize::f32_one")
    )]
    pub roughness_factor: f32,
}

impl GltfPbrMetallicRoughness {
    pub fn base_color_texture(&self) -> &Option<GltfTextureInfo> {
        &self.base_color_texture
    }
    pub fn metallic_roughness_texture(&self) -> &Option<GltfTextureInfo> {
        &self.metallic_roughness_texture
    }
}

//a GltfMaterial
//tp GltfMaterial
/// A type representing a Gltf Material -
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GltfMaterial {
    /// Optional name of the material
    pub name: String,
    /// Image index (source)
    #[cfg_attr(feature = "serde", serde(rename = "normalTexture"))]
    pub normal_texture: Option<GltfTextureInfo>,
    /// occlusion texture
    #[cfg_attr(feature = "serde", serde(rename = "occlusionTexture"))]
    pub occlusion_texture: Option<GltfTextureInfo>,
    /// emissive texture
    #[cfg_attr(feature = "serde", serde(rename = "emissiveTexture"))]
    pub emissive_texture: Option<GltfTextureInfo>,
    /// pbrMetallicRoughness
    #[cfg_attr(feature = "serde", serde(rename = "pbrMetallicRoughness"))]
    pub pbr_metallic_roughness: Option<GltfPbrMetallicRoughness>,

    /// Emissive factor
    #[cfg_attr(feature = "serde", serde(rename = "emissiveFactor"))]
    pub emissive_factor: [f32; 3],

    /// One of OPAQUE, MASK, BLEND
    #[cfg_attr(feature = "serde", serde(rename = "alphaMode"))]
    pub alpha_mode: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "alphaCutoff"))]
    pub alpha_cutoff: f32,
    #[cfg_attr(feature = "serde", serde(rename = "doubleSided"))]
    pub double_sided: bool,
}

impl GltfMaterial {
    pub fn pbr_metallic_roughness(&self) -> &Option<GltfPbrMetallicRoughness> {
        &self.pbr_metallic_roughness
    }
    pub fn normal_texture(&self) -> &Option<GltfTextureInfo> {
        &self.normal_texture
    }
    pub fn occlusion_texture(&self) -> &Option<GltfTextureInfo> {
        &self.occlusion_texture
    }
    pub fn emissive_texture(&self) -> &Option<GltfTextureInfo> {
        &self.emissive_texture
    }
}

//ip Named for GltfMaterial
impl Named for GltfMaterial {
    type Index = MaterialIndex;
    fn is_name(&self, name: &str) -> bool {
        self.name == name
    }
}
