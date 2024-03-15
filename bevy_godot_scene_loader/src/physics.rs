use bevy::ecs::{entity::Entity, system::Commands};

use std::collections::HashMap;

use common::WorldResource;
use serde_json::Value;

// Bevy Rapier Disabled
#[cfg(not(feature = "bevy_rapier"))]
pub fn static_body(commands: &mut Commands) -> Entity {
    commands.spawn(bevy::prelude::SpatialBundle::default()).id()
}

#[cfg(not(feature = "bevy_rapier"))]
pub fn rigid_body(commands: &mut Commands) -> Entity {
    commands.spawn(bevy::prelude::SpatialBundle::default()).id()
}

#[cfg(not(feature = "bevy_rapier"))]
pub fn kinematic_body(commands: &mut Commands) -> Entity {
    commands.spawn(bevy::prelude::SpatialBundle::default()).id()
}

#[cfg(not(feature = "bevy_rapier"))]
pub fn collision_shape(
    commands: &mut Commands,
    _resources: &HashMap<String, WorldResource>,
    _metadata: &HashMap<String, Value>,
    _shape: &String,
) -> Entity {
    commands.spawn(bevy::prelude::SpatialBundle::default()).id()
}

// Bevy Rapier Implementation
#[cfg(feature = "bevy_rapier")]
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider, geometry::Sensor};

#[cfg(feature = "bevy_rapier")]
use common::ResourceData;

#[cfg(feature = "bevy_rapier")]
pub fn create_collider_from_resource(resource: &ResourceData) -> Collider {
    use bevy::math::Vec3;

    match resource {
        ResourceData::SphereCollisionShape(sh) => Collider::ball(sh.radius),
        ResourceData::BoxCollisionShape(sh) => {
            Collider::cuboid(sh.size[0] / 2.0, sh.size[1] / 2.0, sh.size[2] / 2.0)
        }
        ResourceData::ConcavePolygonCollisionShape(sh) => {
            let mut verts = vec![];
            for i in (0..sh.data.len()).step_by(3) {
                verts.push(Vec3::new(sh.data[i], sh.data[i + 1], sh.data[i + 2]));
            }

            Collider::polyline(verts, None)
        }
        _ => panic!("not shape"),
    }
}

#[cfg(feature = "bevy_rapier")]
pub fn static_body(commands: &mut Commands) -> Entity {
    commands.spawn(RigidBody::Fixed).id()
}

#[cfg(feature = "bevy_rapier")]
pub fn rigid_body(commands: &mut Commands) -> Entity {
    commands.spawn(RigidBody::Dynamic).id()
}

#[cfg(feature = "bevy_rapier")]
pub fn kinematic_body(commands: &mut Commands) -> Entity {
    commands.spawn(RigidBody::KinematicVelocityBased).id()
}

#[cfg(feature = "bevy_rapier")]
pub fn collision_shape(
    commands: &mut Commands,
    resources: &HashMap<String, WorldResource>,
    metadata: &HashMap<String, Value>,
    shape: &String,
) -> Entity {
    use bevy::ecs::system::EntityCommands;

    let resource = resources.get(shape).unwrap();
    let collider = create_collider_from_resource(&resource.data);

    let mut builder: &mut EntityCommands = &mut commands.spawn(collider);

    if let Some(sensor_value) = metadata.get("sensor") {
        if let Some(true) = sensor_value.as_bool() {
            builder = builder.insert(Sensor);
        }
    }

    builder.id()
}
