use bevy::{math::Mat4, transform::components::Transform};
use common::EntityData;

pub fn strip_res_prefix(str: &String) -> String {
    return str.replace("res://", "");
}

pub fn vec_to_transform(transform: &Vec<f32>) -> Transform {
    let slice = &transform[0..16];
    Transform::from_matrix(Mat4::from_cols_array(slice.try_into().unwrap()))
}

macro_rules! transform {
    ($var:expr) => {
        Some(vec_to_transform(&$var.transform))
    };
}

pub fn get_transform_from_data(data: &EntityData) -> Option<Transform> {
    match data {
        EntityData::MeshInstance3D(data) => transform!(data),
        EntityData::Node3D(data) => transform!(data),
        EntityData::StaticBody3D(data) => transform!(data),
        EntityData::RigidBody3D(data) => transform!(data),
        EntityData::KinematicBody3D(data) => transform!(data),
        EntityData::CollisionShape3D(data) => transform!(data),
        EntityData::ModelScene(data) => transform!(data),
        EntityData::Camera(data) => transform!(data),
    }
}
