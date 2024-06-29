//a Imports
use serde::Deserialize;

use crate::Named;
use crate::{GltfTextureInfo, MaterialIndex};

fn f32_one() -> f32 {
    1.0
}

//tp GltfPbrMetallicRoughness
///
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GltfPbrMetallicRoughness {
    /// Base color factor
    #[serde(rename = "baseColorFactor")]
    pub base_color_factor: Option<Vec<f32>>,
    #[serde(rename = "baseColorTexture")]
    pub base_color_texture: Option<GltfTextureInfo>,
    #[serde(rename = "metallicRoughnessTexture")]
    pub metallic_roughness_texture: Option<GltfTextureInfo>,
    #[serde(rename = "metallicFactor", default = "f32_one")]
    pub metallic_factor: f32,
    #[serde(rename = "roughnessFactor", default = "f32_one")]
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
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GltfMaterial {
    /// Optional name of the material
    pub name: String,
    /// Image index (source)
    #[serde(rename = "normalTexture")]
    pub normal_texture: Option<GltfTextureInfo>,
    /// occlusion texture
    #[serde(rename = "occlusionTexture")]
    pub occlusion_texture: Option<GltfTextureInfo>,
    /// emissive texture
    #[serde(rename = "emissiveTexture")]
    pub emissive_texture: Option<GltfTextureInfo>,
    /// pbrMetallicRoughness
    #[serde(rename = "pbrMetallicRoughness")]
    pub pbr_metallic_roughness: Option<GltfPbrMetallicRoughness>,

    /// Emissive factor
    #[serde(rename = "emissiveFactor")]
    pub emissive_factor: [f32; 3],

    /// One of OPAQUE, MASK, BLEND
    #[serde(rename = "alphaMode")]
    pub alpha_mode: Option<String>,
    #[serde(rename = "alphaCutoff")]
    pub alpha_cutoff: f32,
    #[serde(rename = "doubleSided")]
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
