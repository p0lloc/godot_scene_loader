use std::collections::HashMap;

use common::{
    entities::physics::CollisionShapeData, get_or_return_val, EntityData, ResourceData,
    WorldEntity, WorldResource,
};
pub use common::{load_scene_world_file, SceneWorld};
use rapier3d::{
    dynamics::{IslandManager, RigidBodyBuilder, RigidBodyHandle, RigidBodySet, RigidBodyType},
    geometry::{ActiveCollisionTypes, Collider, ColliderBuilder, ColliderHandle, ColliderSet},
    na::{Isometry3, Matrix3, Matrix4, Point3, Rotation3, UnitQuaternion, Vector3, Vector4},
    pipeline::ActiveEvents,
};
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct NodeTransform {
    pub matrix: Matrix4<f32>,

    pub rotation: UnitQuaternion<f32>,
    pub translation: Vector3<f32>,
}

impl From<NodeTransform> for Isometry3<f32> {
    fn from(transform: NodeTransform) -> Self {
        return Isometry3 {
            translation: transform.translation.into(),
            rotation: transform.rotation,
        };
    }
}

impl From<&NodeTransform> for Isometry3<f32> {
    fn from(transform: &NodeTransform) -> Self {
        return Isometry3 {
            translation: transform.translation.into(),
            rotation: transform.rotation,
        };
    }
}

#[derive(Clone, Debug)]
pub enum SpawnedWorldEntityData {
    PhysicsBody((RigidBodyHandle, RigidBodyType)),
    Collider(ColliderHandle),
    Node,
}

pub struct SpawnedWorldEntity {
    pub entity_type: String,
    pub data: SpawnedWorldEntityData,
    pub transform: NodeTransform,
    pub metadata: HashMap<String, Value>,
}

impl NodeTransform {
    pub fn from_matrix(matrix: &Matrix4<f32>) -> Self {
        let last_column: Vector4<f32> = matrix.column(3).into();
        let translation: Vector3<f32> =
            Vector3::new(last_column[0], last_column[1], last_column[2]);

        let rotation_view: Matrix3<f32> = matrix.fixed_view::<3, 3>(0, 0).into();
        let rotation =
            UnitQuaternion::from_rotation_matrix(&Rotation3::from_matrix(&rotation_view));

        return Self {
            matrix: matrix.clone(),
            translation,
            rotation,
        };
    }
}

impl Default for NodeTransform {
    fn default() -> Self {
        return Self {
            matrix: Matrix4::default(),
            rotation: UnitQuaternion::default(),
            translation: Vector3::default(),
        };
    }
}

pub fn load_world_to_rapier(
    world: &SceneWorld,
    transform: Option<Matrix4<f32>>,
) -> (
    RigidBodySet,
    ColliderSet,
    IslandManager,
    HashMap<String, SpawnedWorldEntity>,
) {
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut islands = IslandManager::new();
    let mut entities: HashMap<String, SpawnedWorldEntity> = HashMap::new();

    for entity in &world.entities {
        spawn_entity(
            &entity,
            transform.unwrap_or(Matrix4::identity()),
            None,
            &mut bodies,
            &mut colliders,
            &mut islands,
            &world.resources,
            &mut entities,
        );
    }

    return (bodies, colliders, islands, entities);
}

fn get_entity_transform(entity: &WorldEntity) -> Option<Matrix4<f32>> {
    let data: Option<&Vec<f32>> = match &entity.data {
        EntityData::StaticBody3D(body) => Some(&body.transform),
        EntityData::RigidBody3D(body) => Some(&body.transform),
        EntityData::KinematicBody3D(body) => Some(&body.transform),
        EntityData::CollisionShape3D(shape) => Some(&shape.transform),
        EntityData::Node3D(body) => Some(&body.transform),
        EntityData::ModelScene(body) => Some(&body.transform),
        _ => None,
    };

    return data.and_then(|x| {
        return Some(Matrix4::from_column_slice(&x));
    });
}

fn spawn_entity(
    entity: &WorldEntity,
    parent_transform: Matrix4<f32>,
    parent_data: Option<&SpawnedWorldEntityData>,

    bodies: &mut RigidBodySet,
    colliders: &mut ColliderSet,
    islands: &mut IslandManager,
    resources: &HashMap<String, WorldResource>,
    entities: &mut HashMap<String, SpawnedWorldEntity>,
) -> Option<SpawnedWorldEntityData> {
    let relative_transform = get_or_return_val!(get_entity_transform(entity), None);
    let absolute_transform = parent_transform * relative_transform;
    let node_transform = NodeTransform::from_matrix(&absolute_transform);

    let data = spawn_entity_data(
        entity,
        parent_data,
        node_transform,
        &relative_transform,
        bodies,
        colliders,
        resources,
        entities,
    );

    if let Some(children) = &entity.children {
        for child in children {
            spawn_entity(
                child,
                absolute_transform,
                data.as_ref(),
                bodies,
                colliders,
                islands,
                resources,
                entities,
            );
        }
    }

    return data;
}

pub fn spawn_body(
    body_type: RigidBodyType,
    transform: &NodeTransform,
    bodies: &mut RigidBodySet,
) -> RigidBodyHandle {
    let rb = RigidBodyBuilder::new(body_type)
        .position(transform.into())
        .build();

    return bodies.insert(rb);
}

fn spawn_collision_shape(
    entity: &WorldEntity,
    shape: &CollisionShapeData,
    absolute_transform: &NodeTransform,
    relative_transform: &Matrix4<f32>,
    colliders: &mut ColliderSet,
    bodies: &mut RigidBodySet,

    parent_data: Option<&SpawnedWorldEntityData>,
    resources: &HashMap<String, WorldResource>,
) -> Option<SpawnedWorldEntityData> {
    if let Some(parent_data) = parent_data {
        if let SpawnedWorldEntityData::PhysicsBody((parent_handle, parent_body_type)) = parent_data
        {
            let mut collider: Collider = if let Some(col) =
                parse_collider(resources, shape, Some(parent_body_type), &entity.metadata)
            {
                col
            } else {
                return None;
            };

            // Use transform relative to the parent body
            let pos = NodeTransform::from_matrix(relative_transform);
            collider.set_position(pos.into());

            let handle = colliders.insert_with_parent(collider, parent_handle.clone(), bodies);
            return Some(SpawnedWorldEntityData::Collider(handle));
        }
    }

    let mut collider: Collider =
        if let Some(col) = parse_collider(resources, shape, None, &entity.metadata) {
            col
        } else {
            return None;
        };

    collider.set_position(absolute_transform.into());
    let handle = colliders.insert(collider);

    return Some(SpawnedWorldEntityData::Collider(handle));
}

fn parse_collider(
    resources: &HashMap<String, WorldResource>,
    shape: &CollisionShapeData,
    _parent_body_type: Option<&RigidBodyType>,
    metadata: &HashMap<String, Value>,
) -> Option<Collider> {
    let res = get_or_return_val!(resources.get(&shape.shape), None);

    let mut collider_builder = match &res.data {
        ResourceData::BoxCollisionShape(shape) => ColliderBuilder::cuboid(
            shape.size[0] / 2.0,
            shape.size[1] / 2.0,
            shape.size[2] / 2.0,
        ),
        ResourceData::SphereCollisionShape(shape) => ColliderBuilder::ball(shape.radius),
        ResourceData::ConcavePolygonCollisionShape(shape) => {
            let mut verts = vec![];
            for i in (0..shape.data.len()).step_by(3) {
                verts.push(Point3::new(
                    shape.data[i],
                    shape.data[i + 1],
                    shape.data[i + 2],
                ));
            }
            ColliderBuilder::polyline(verts, None)
        }
        _ => {
            panic!("invalid shape");
        }
    };

    if let Some(sensor_value) = metadata.get("sensor") {
        if let Some(sensor) = sensor_value.as_bool() {
            collider_builder = collider_builder
                .sensor(sensor)
                .active_collision_types(ActiveCollisionTypes::all())
                .active_events(ActiveEvents::all());
        }
    }

    return Some(collider_builder.build());
}

fn spawn_entity_data(
    entity: &WorldEntity,
    parent_data: Option<&SpawnedWorldEntityData>,
    absolute_transform: NodeTransform,
    relative_transform: &Matrix4<f32>,
    bodies: &mut RigidBodySet,
    colliders: &mut ColliderSet,
    resources: &HashMap<String, WorldResource>,
    entities: &mut HashMap<String, SpawnedWorldEntity>,
) -> Option<SpawnedWorldEntityData> {
    let body_type = match &entity.data {
        EntityData::StaticBody3D(_) => Some(RigidBodyType::Fixed),
        EntityData::KinematicBody3D(_) => Some(RigidBodyType::KinematicVelocityBased),
        EntityData::RigidBody3D(_) => Some(RigidBodyType::Dynamic),
        _ => None,
    };

    let data = if let Some(body_type) = body_type {
        let handle = spawn_body(body_type, &absolute_transform, bodies);
        Some(SpawnedWorldEntityData::PhysicsBody((handle, body_type)))
    } else {
        match &entity.data {
            EntityData::CollisionShape3D(shape) => spawn_collision_shape(
                entity,
                shape,
                &absolute_transform,
                relative_transform,
                colliders,
                bodies,
                parent_data,
                resources,
            ),
            _ => None,
        }
    };

    if let Some(data) = &data {
        entities.insert(
            entity.name.clone(),
            SpawnedWorldEntity {
                entity_type: entity.entity_type.clone(),
                transform: absolute_transform.clone(),
                metadata: entity.metadata.clone(),
                data: data.clone(),
            },
        );
    }

    return data;
}
