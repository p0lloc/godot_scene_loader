use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BoxCollisionShapeData {
    pub size: Vec<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SphereCollisionShapeData {
    pub radius: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConcavePolygonCollisionShapeData {
    pub data: Vec<f32>,
}
