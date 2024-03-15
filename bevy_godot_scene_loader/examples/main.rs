use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_godot_scene_loader::load_scene_to_bevy;
use common::load_scene_world_file;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let world = load_scene_world_file("bevy_godot_scene_loader/examples/test-world.json");
    load_scene_to_bevy(&world, &mut commands, &mut meshes, &mut materials, &assets);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0),
        ..default()
    });
}

fn movement_system(
    mut camera_transform: Query<&mut Transform, With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut transform = camera_transform.get_single_mut().unwrap();
    if input.pressed(KeyCode::KeyW) {
        transform.translation += Vec3::new(0.0, 0.0, -0.3);
    }

    if input.pressed(KeyCode::KeyS) {
        transform.translation += Vec3::new(0.0, 0.0, 0.3);
    }

    if input.pressed(KeyCode::KeyA) {
        transform.translation += Vec3::new(-0.3, 0.0, 0.0);
    }

    if input.pressed(KeyCode::KeyD) {
        transform.translation += Vec3::new(0.3, 0.0, 0.0);
    }

    if input.pressed(KeyCode::Space) {
        transform.translation += Vec3::new(0.0, 0.3, 0.0);
    }

    if input.pressed(KeyCode::ShiftLeft) {
        transform.translation += Vec3::new(0.0, -0.3, 0.0);
    }
}
