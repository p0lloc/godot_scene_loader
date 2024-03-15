use std::collections::HashMap;

use entities::{
    node::{CameraData, Node3DData},
    physics::{CollisionShapeData, KinematicBodyData, RigidBodyData, StaticBodyData},
    render::{MeshInstanceData, ModelSceneData},
};
use resources::{
    physics::{BoxCollisionShapeData, ConcavePolygonCollisionShapeData, SphereCollisionShapeData},
    render::{
        ArrayMeshData, BoxMeshData, PackedSceneData, SphereMeshData, StandardMaterialData,
        Texture2DData,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod entities;
pub mod macros;
pub mod resources;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WorldEntityJson {
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub entity_type: String,
    pub data: Value,
    pub metadata: HashMap<String, Value>,
    pub children: Option<Vec<WorldEntityJson>>,
}

macro_rules! serde_deser {
    ($var:expr) => {
        serde_json::from_value($var.clone()).unwrap()
    };
}

impl WorldEntityJson {
    pub fn parse_data(&self) -> EntityData {
        return match self.entity_type.as_str() {
            "StaticBody3D" => EntityData::StaticBody3D(serde_deser!(self.data)),
            "MeshInstance3D" => EntityData::MeshInstance3D(serde_deser!(self.data)),
            "CollisionShape3D" => EntityData::CollisionShape3D(serde_deser!(self.data)),
            "Camera3D" => EntityData::Camera(serde_deser!(self.data)),
            "RigidBody3D" => EntityData::RigidBody3D(serde_deser!(self.data)),
            "Node3D" => EntityData::Node3D(serde_deser!(self.data)),
            "CharacterBody3D" => EntityData::KinematicBody3D(serde_deser!(self.data)),
            "" => {
                return EntityData::ModelScene(serde_deser!(self.data));
            }
            _ => panic!("parsing invalid entity type {}", self.entity_type.as_str()),
        };
    }

    pub fn parse(&self) -> WorldEntity {
        WorldEntity {
            name: self.name.clone(),
            entity_type: self.entity_type.clone(),
            data: self.parse_data(),
            metadata: self.metadata.clone(),
            children: self
                .children
                .clone()
                .and_then(|x| Some(x.iter().map(|x| x.parse()).collect())),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WorldEntity {
    pub name: String,
    pub entity_type: String,
    pub data: EntityData,
    pub metadata: HashMap<String, Value>,
    pub children: Option<Vec<WorldEntity>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum EntityData {
    StaticBody3D(StaticBodyData),
    RigidBody3D(RigidBodyData),
    KinematicBody3D(KinematicBodyData),
    CollisionShape3D(CollisionShapeData),

    ModelScene(ModelSceneData),
    MeshInstance3D(MeshInstanceData),

    Camera(CameraData),
    Node3D(Node3DData),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ResourceData {
    BoxMesh(BoxMeshData),
    SphereMesh(SphereMeshData),
    ArrayMesh(ArrayMeshData),

    StandardMaterial(StandardMaterialData),
    Texture2D(Texture2DData),

    BoxCollisionShape(BoxCollisionShapeData),
    SphereCollisionShape(SphereCollisionShapeData),
    ConcavePolygonCollisionShape(ConcavePolygonCollisionShapeData),

    PackedScene(PackedSceneData),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WorldResourceJson {
    #[serde(rename(deserialize = "type"))]
    resource_type: String,
    data: Value,
}

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct WorldResource {
    pub data: ResourceData,
}

impl WorldResourceJson {
    pub fn parse(&self) -> WorldResource {
        return WorldResource {
            data: self.parse_data(),
        };
    }

    pub fn parse_data(&self) -> ResourceData {
        return match self.resource_type.as_str() {
            "BoxMesh" => ResourceData::BoxMesh(serde_deser!(self.data)),
            "SphereMesh" => ResourceData::SphereMesh(serde_deser!(self.data)),
            "StandardMaterial3D" => ResourceData::StandardMaterial(serde_deser!(self.data)),
            "ConcavePolygonShape3D" => {
                ResourceData::ConcavePolygonCollisionShape(serde_deser!(self.data))
            }
            "BoxShape3D" => ResourceData::BoxCollisionShape(serde_deser!(self.data)),
            "SphereShape3D" => ResourceData::SphereCollisionShape(serde_deser!(self.data)),
            "ArrayMesh" => {
                let path = self
                    .data
                    .as_str()
                    .expect("ExtResource data should always be string");

                ResourceData::ArrayMesh(ArrayMeshData {
                    path: path.to_owned(),
                })
            }
            "Texture2D" => {
                let path = self
                    .data
                    .as_str()
                    .expect("ExtResource data should always be string");

                ResourceData::Texture2D(Texture2DData {
                    path: path.to_owned(),
                })
            }

            "PackedScene" => {
                let path = self
                    .data
                    .as_str()
                    .expect("ExtResource data should always be string");

                ResourceData::PackedScene(PackedSceneData {
                    path: path.to_owned(),
                })
            }
            _ => panic!("invalid resource type {}", self.resource_type.as_str()),
        };
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SceneWorldJson {
    pub entities: Vec<WorldEntityJson>,
    pub resources: HashMap<String, WorldResourceJson>,
}

impl SceneWorldJson {
    pub fn to_world(&self) -> SceneWorld {
        return SceneWorld {
            entities: self.entities.iter().map(|x| x.parse()).collect(),
            resources: self
                .resources
                .iter()
                .map(|(k, v)| (k.clone(), v.parse()))
                .collect(),
        };
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SceneWorld {
    pub entities: Vec<WorldEntity>,
    pub resources: HashMap<String, WorldResource>,
}

pub fn load_scene_world_file(file: &str) -> SceneWorld {
    let file = std::fs::File::open(file).unwrap();
    let json: SceneWorldJson = serde_json::from_reader(file).expect("file should be proper JSON");

    json.to_world()
}
