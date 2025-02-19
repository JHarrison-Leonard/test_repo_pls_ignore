//! little guy

use avian3d::prelude::*;
use bevy::{
    color::palettes::css::PINK,
    prelude::*,
};



pub struct LittleGuyPlugin;

impl Plugin for LittleGuyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}



/// Spawn a little guy
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.2, 0.5))),
        MeshMaterial3d(materials.add(Color::from(PINK))),
        RigidBody::Dynamic,
        Collider::capsule(0.2, 0.5),
        ColliderDensity(1.0),
        Transform::from_xyz(5.0, 5.0, 5.0),
    ));
}
