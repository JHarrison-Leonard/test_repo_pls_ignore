//! Game plugin for the "game"
//! There's definitely a game in this code
//! Just gotta take our word for it

use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::*,
    prelude::*,
};

//mod ominous_cone;
//mod little_guy;



pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
//            .add_plugin(OminousConePlugin)
//            .add_plugin(LittleGuyPlugin)
            .add_systems(Startup, setup)
            .add_systems(Startup, disable_ambient_lighting);
    }
}



/// Set up default scene for testing
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
    commands.spawn(
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



/// Disables ambient lighting, for the vibes
fn disable_ambient_lighting(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 0.0;
}
