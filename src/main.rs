// https://chatgpt.com/c/66fab020-e804-8013-8620-e311d93778a5?conversationId=66fab020-e804-8013-8620-e311d93778a5
// https://crates.io/crates/gltf
//https://en.wikipedia.org/wiki/Protein_Data_Bank_(file_format)

use bevy::{
    input::{mouse::MouseMotion, mouse::MouseWheel, Input},
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use std::f32::consts::*;

const MAX_ANGLE_Y: f32 = FRAC_PI_2 - 1e-6;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_controller)
        .run();
}

// Camera control component to track rotation and zoom level
#[derive(Component)]
struct CameraController {
    pub radius: f32,
    pub angle_x: f32,
    pub angle_y: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the camera with an initial CameraController
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.7, 0.7, 1.0)
                .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ..default()
        },
        CameraController {
            radius: 100.0,
            angle_x: 0.0,
            angle_y: 0.0,
        },
    ));

    // Spawn the directional light with shadows
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });

    // Load and spawn the scene from the `matilda.glb` file
    commands.spawn(SceneBundle {
        scene: asset_server.load("matilda.glb#Scene0"),
        ..default()
    });
}

// System for controlling the camera
fn camera_controller(
    _time: Res<Time>,
    mut motion_event_reader: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut query: Query<(&mut CameraController, &mut Transform)>,
) {
    let (mut controller, mut transform) = query.single_mut();

    // Check if the left mouse button is pressed to enable rotation
    if mouse_buttons.pressed(MouseButton::Left) {
        for event in motion_event_reader.read() {
            // Update camera angles based on mouse movement
            controller.angle_x -= event.delta.x * 0.005;
            controller.angle_y =
                (controller.angle_y - event.delta.y * 0.005).clamp(-MAX_ANGLE_Y, MAX_ANGLE_Y);
        }
    }

    // Zoom control using mouse scroll
    for event in scroll_events.read() {
        controller.radius = (controller.radius * (0.85_f32).powf(event.y)).clamp(0.01, 1000.0);
    }

    // Update camera position based on the new angles and radius
    let x = controller.radius * controller.angle_x.cos() * controller.angle_y.cos();
    let y = controller.radius * controller.angle_y.sin();
    let z = controller.radius * controller.angle_x.sin() * controller.angle_y.cos();

    // Update the camera transform to "orbit" around the center (0, 0, 0)
    transform.translation = Vec3::new(x, y, z);
    transform.look_at(Vec3::ZERO, Vec3::Y);
}
