//! little guy

use avian3d::prelude::*;
use bevy::{
    color::palettes::css::PINK,
    prelude::*,
};

use crate::game::ominous_cone::OminousCone;



pub struct LittleGuyPlugin;

impl Plugin for LittleGuyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (get_going, go_to_target));
    }
}



#[derive(Component)]
pub struct LittleGuy {
    going: bool,
    target: Vec3,
}

impl LittleGuy {
    pub const fn new() -> Self {
        return LittleGuy {
            going: false,
            target: Vec3::new(0.0, 0.0, 0.0),
        };
    }
}



/// Spawn a little guy
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
            LittleGuy::new(),
            Mesh3d(meshes.add(Capsule3d::new(0.2, 0.5))),
            MeshMaterial3d(materials.add(Color::from(PINK))),
            RigidBody::Dynamic,
            Collider::capsule(0.2, 0.5),
            ColliderDensity(1.0),
            Transform::from_xyz(5.0, 5.0, 5.0),
            LockedAxes::new()
                .lock_rotation_x()
                .lock_rotation_z(),
    ));
}



fn get_going(
    mut query: Query<&mut LittleGuy>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut guy in &mut query {
            guy.going = true;
        }
    }
}



fn go_to_target(
    mut guys: Query<(&mut LittleGuy, &mut ExternalForce, &GlobalTransform), (With<LittleGuy>, Without<OminousCone>)>,
    targets: Query<&GlobalTransform, (With<OminousCone>, Without<LittleGuy>)>,
) {
    for (mut guy, mut force, guy_transform) in &mut guys {
        if guy.going {
            // Find closest cone
            for target_transform in &targets {
                let new_target = target_transform.translation() - guy_transform.translation();
                if (guy.target.length() == 0.0) || (new_target.length() < guy.target.length()) {
                    guy.target = new_target;
                }
            }

            // Apply force
            force.apply_force(guy.target);
        }
    }
}
