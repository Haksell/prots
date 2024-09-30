//! Loads and renders the `matilda.glb` file as a scene, and adds a camera controller for orbiting with the mouse.

use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::{
    input::{
        mouse::{MouseMotion, MouseWheel},
        Input,
    },
    prelude::*,
    window::CursorGrabMode,
};
use std::f32::consts::*;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_light_direction)
        .add_systems(Update, camera_controller) // Add camera control system
        .run();
}

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
            radius: 3.0,  // Initial distance from the model
            angle_x: 0.0, // Horizontal angle
            angle_y: 0.0, // Vertical angle
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
        scene: asset_server.load("models/matilda.glb#Scene0"),
        ..default()
    });
}

// Animate the directional light rotation
fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}

// System for controlling the camera
fn camera_controller(
    time: Res<Time>,
    mut motion_event_reader: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut query: Query<(&mut CameraController, &mut Transform)>,
) {
    let (mut controller, mut transform) = query.single_mut();

    // Check if the left mouse button is pressed to enable rotation
    if mouse_buttons.pressed(MouseButton::Left) {
        for event in motion_event_reader.iter() {
            // Update camera angles based on mouse movement
            controller.angle_x -= event.delta.x * 0.005;
            controller.angle_y =
                (controller.angle_y - event.delta.y * 0.005).clamp(-PI / 2.0, PI / 2.0);
        }
    }

    // Zoom control using mouse scroll
    for event in scroll_events.iter() {
        controller.radius = (controller.radius - event.y * 0.2).clamp(1.0, 10.0);
    }

    // Update camera position based on the new angles and radius
    let x = controller.radius * controller.angle_x.cos() * controller.angle_y.cos();
    let y = controller.radius * controller.angle_y.sin();
    let z = controller.radius * controller.angle_x.sin() * controller.angle_y.cos();

    // Update the camera transform to "orbit" around the center (0, 0, 0)
    transform.translation = Vec3::new(x, y, z);
    transform.look_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y);
}
