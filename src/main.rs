//! Avalanche Living
//! Loosely based on beloved advertisement, ******** ********
//! Fun Fact: I don't know what I'm doing. At all.

use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::*,
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
        ))
        .add_systems(Startup, disable_ambient_lighting)
        .add_systems(Startup, setup)
        .run();
}



/// Disables ambient lighting
fn disable_ambient_lighting(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 0.0;
}



/// Set's up default scene for testing
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn ground plane
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(20.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(WHITE))),
        RigidBody::Static,
        Collider::cylinder(20.0, 1.0),
    ));

    // Spawn fire
    commands.spawn((
        PointLight {
            color: Color::from(RED),
            shadows_enabled: true,
            intensity: 100_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Mesh3d(meshes.add(Cone::default())),
        MeshMaterial3d(materials.add(Color::from(RED))),
        Transform::from_xyz(0.0, 5.0, 0.0),
        RigidBody::Dynamic,
        Collider::cone(0.5, 1.0),
        ColliderDensity(1.0),
    ));

    // Spawn light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 100_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // Spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 16.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));
}

