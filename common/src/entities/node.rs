use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Node3DData {
    pub transform: Vec<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CameraData {
    pub transform: Vec<f32>,
}
