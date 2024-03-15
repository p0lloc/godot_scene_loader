use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CollisionShapeData {
    pub shape: String,
    pub transform: Vec<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StaticBodyData {
    pub transform: Vec<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct KinematicBodyData {
    pub transform: Vec<f32>,
    #[serde(rename = "linearVelocity")]
    pub linear_velocity: Option<Vec<f32>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RigidBodyData {
    pub transform: Vec<f32>,

    #[serde(rename = "linearVelocity")]
    pub linear_velocity: Option<Vec<f32>>,

    #[serde(rename = "angularVelocity")]
    pub angular_velocity: Option<Vec<f32>>,
}
