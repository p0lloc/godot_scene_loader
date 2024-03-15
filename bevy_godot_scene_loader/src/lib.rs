use std::collections::HashMap;

use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::{
        entity::Entity,
        system::{Commands, Res, ResMut},
    },
    hierarchy::BuildChildren,
    pbr::{AlphaMode, PbrBundle, StandardMaterial},
    prelude::{default, SpatialBundle},
    render::{mesh::Mesh, view::Visibility},
    scene::{Scene, SceneBundle},
    transform::components::Transform,
};
pub use common::{load_scene_world_file, SceneWorld, SceneWorldJson};
use common::{EntityData, ResourceData, WorldEntity};
use mesh::{create_mesh_from_resource, MaterialInfo, MeshInfo};
use physics::{kinematic_body, rigid_body, static_body};
use util::{get_transform_from_data, strip_res_prefix};

pub mod mesh;
pub mod physics;
pub mod util;

pub struct SpawnedEntity {
    pub id: Entity,
    pub entity_type: String,
}

/// Loads a [SceneWorld] into Bevy by spawning all the entities in Bevy format.
pub fn load_scene_to_bevy(
    world: &SceneWorld,

    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &Res<AssetServer>,
) -> HashMap<String, SpawnedEntity> {
    let mut spawned_entities = HashMap::new();
    for entity in &world.entities {
        spawn_entity(
            world,
            entity,
            commands,
            meshes,
            materials,
            assets,
            &mut spawned_entities,
        );
    }

    return spawned_entities;
}

/// Spawns a [WorldEntity] from [SceneWorld] into the Bevy scene.
pub fn spawn_entity(
    world: &SceneWorld,
    entity: &WorldEntity,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &Res<AssetServer>,
    mut spawned_entities: &mut HashMap<String, SpawnedEntity>,
) -> Option<Entity> {
    let relative_transform = get_transform_from_data(&entity.data).unwrap_or(Transform::IDENTITY);

    // Spawn the components for this entity
    let entity_id = if let Some(id) = spawn_components(
        world,
        entity,
        relative_transform,
        commands,
        meshes,
        materials,
        assets,
    ) {
        id
    } else {
        return None;
    };

    spawned_entities.insert(
        entity.name.clone(),
        SpawnedEntity {
            id: entity_id,
            entity_type: entity.entity_type.clone(),
        },
    );

    // Spawn the children of this entity and add them as Bevy children.
    if let Some(children) = &entity.children {
        for child in children {
            if let Some(child_id) = spawn_entity(
                world,
                &child,
                commands,
                meshes,
                materials,
                assets,
                &mut spawned_entities,
            ) {
                commands.entity(entity_id).add_child(child_id);
            }
        }
    }

    Some(entity_id)
}

/// Spawns the components that this entity corresponds to in Bevy format.
/// Returns the spawned entity ID if it was successful.
pub fn spawn_components(
    world: &SceneWorld,
    entity: &WorldEntity,
    transform: Transform,
    mut commands: &mut Commands,

    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &Res<AssetServer>,
) -> Option<Entity> {
    match &entity.data {
        EntityData::StaticBody3D(_) => {
            let entity = static_body(&mut commands);
            Some(commands.entity(entity).insert(transform).id())
        }

        EntityData::RigidBody3D(_) => {
            let entity = rigid_body(&mut commands);
            Some(commands.entity(entity).insert(transform).id())
        }

        EntityData::KinematicBody3D(_) => {
            let entity = kinematic_body(&mut commands);

            Some(commands.entity(entity).insert(transform).id())
        }
        EntityData::Node3D(_) => Some(
            commands
                .spawn(SpatialBundle::default())
                .insert(transform)
                .id(),
        ),
        EntityData::CollisionShape3D(_) => Some(
            commands
                .spawn(SpatialBundle::default())
                .insert(transform)
                .id(),
        ),
        EntityData::MeshInstance3D(instance) => {
            let mesh = create_mesh_from_resource(instance.mesh.clone(), &world.resources, &assets);

            // Create the material for this mesh
            let material = match mesh.material {
                MaterialInfo::Texture(tex) => materials.add(StandardMaterial {
                    base_color_texture: Some(tex.clone()),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                }),
                MaterialInfo::Material(mat) => materials.add(mat),
            };

            // Create the actual mesh
            let handle = match mesh.mesh {
                MeshInfo::Mesh(mh) => meshes.add(mh),
                MeshInfo::ArrayMesh(am) => am,
            };

            // Component for if this mesh should be visible or not
            let visibility = if instance.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };

            Some(
                commands
                    .spawn(PbrBundle {
                        mesh: handle,
                        material,
                        ..default()
                    })
                    .insert(visibility)
                    .insert(transform)
                    .id(),
            )
        }
        EntityData::ModelScene(scene) => match scene.type_name.as_str() {
            "MODEL" => {
                if let Some(path) = scene.data.as_str() {
                    // Get resource from path
                    if let Some(resource) = world.resources.get(path) {
                        // Resource must be of type PackedScene
                        if let ResourceData::PackedScene(scene) = &resource.data {
                            let mut path = strip_res_prefix(&scene.path);
                            path = format!("{}#Scene0", path); // Use the first scene

                            let scene_handle: Handle<Scene> = assets.load(path);
                            return Some(
                                commands
                                    .spawn(SceneBundle {
                                        scene: scene_handle,
                                        transform,
                                        ..Default::default()
                                    })
                                    .id(),
                            );
                        }
                    }
                }

                None
            }
            _ => None,
        },
        _ => None,
    }
}
