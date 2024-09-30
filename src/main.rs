use bevy::gltf::Gltf;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Bevy's default plugins, including GLTF support
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut gltf_assets: ResMut<Assets<Gltf>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load the .glb file
    let scene_handle = asset_server.load("models/matilda.glb#Scene0");

    // Spawn the 3D model
    commands.spawn(SceneBundle {
        scene: scene_handle.clone(),
        ..default()
    });

    // Set up a camera to view the 3D model
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add a light source
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
