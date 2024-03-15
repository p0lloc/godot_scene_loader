use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SphereMeshData {
    pub radius: f32,
    pub material: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BoxMeshData {
    pub size: Vec<f32>,
    pub material: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ArrayMeshData {
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Texture2DData {
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StandardMaterialData {
    #[serde(rename = "albedoColor")]
    pub albedo_color: Vec<f32>,

    #[serde(rename = "albedoTexture")]
    pub albedo_texture: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PackedSceneData {
    pub path: String,
}
