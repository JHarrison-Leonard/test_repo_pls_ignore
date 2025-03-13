//! Game plugin for the "game"
//! There's definitely a game in this code
//! Just gotta take our word for it

use avian3d::prelude::*;
use bevy::{
    color::palettes::basic::WHITE,
    prelude::*,
};
use bevy_panorbit_camera::*;
use bevy_spectator::*;
use bevy_water::*;

use ominous_cone::OminousConePlugin;
use little_guy::LittleGuyPlugin;

mod ominous_cone;
mod little_guy;



pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                PhysicsPlugins::default(),
                OminousConePlugin,
                LittleGuyPlugin,
                SpectatorPlugin,
                PanOrbitCameraPlugin,
        ))
            .insert_resource(WaterSettings {
                height: 0.0,
                ..default()
            })
            .add_plugins((WaterPlugin, ImageUtilsPlugin))
            .add_systems(Startup, setup)
            .add_systems(Startup, disable_ambient_lighting)
            .add_systems(Startup, setup_camera);
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
}



fn setup_camera(mut commands: Commands) {
    make_camera(&mut commands);
}



fn make_camera<'a>(commands: &'a mut Commands) -> EntityCommands<'a> {
    // Code mostly stolen from the pirates example in bevy_water
    let mut cam = commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 8.0, 16.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            DistanceFog {
                color: Color::srgba(0.1, 0.2, 0.4, 1.0),
                falloff: FogFalloff::from_visibility_colors(
                    400.0,
                    Color::srgb(0.35, 0.5, 0.66),
                    Color::srgb(0.8, 0.844, 1.0),
                ),
                ..default()
            },
    ));

    cam.insert(Spectator);

    cam.insert(PanOrbitCamera {
        focus: Vec3::new(0., 0., 0.),
        radius: Some(60.0),
        yaw: Some(-std::f32::consts::FRAC_PI_2),
        pitch: Some(0.0),
        ..default()
    });

    cam
}



/// Disables ambient lighting, for the vibes
fn disable_ambient_lighting(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 0.0;
}
