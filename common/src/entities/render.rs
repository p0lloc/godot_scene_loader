use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MeshInstanceData {
    pub mesh: String,
    pub visible: bool,
    pub transform: Vec<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ModelSceneData {
    #[serde(rename(deserialize = "type"))]
    pub type_name: String,
    pub data: Value,
    pub transform: Vec<f32>,
}
