//! O M I N O U S  C O N E

use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::RED,
    prelude::*,
};



pub struct OminousConePlugin;

impl Plugin for OminousConePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}



#[derive(Component)]
pub struct OminousCone;



/// Spawns the O M I N O U S  C O N E
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // SPAWN IT
    commands.spawn((
            OminousCone,
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
}
