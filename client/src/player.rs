use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::math::primitives::Capsule3d;

use crate::level::{MAZE, MAP_WIDTH, MAP_HEIGHT};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

const PLAYER_SPEED: f32 = 6.0;
const MOUSE_SENSITIVITY: f32 = 0.15;
const CAMERA_HEIGHT: f32 = 1.6;

pub fn setup_player_systems(app: &mut App) {
    app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (player_movement, player_camera_look));
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cell_size = 2.0;
    // Find first available corner (top-left) empty cell
    let (spawn_x, spawn_z) = find_spawn_in_corner((0, 0, 1, 1));
    let world_x = spawn_x * cell_size - (MAP_WIDTH as f32 * cell_size) / 2.0 + cell_size / 2.0;
    let world_z = spawn_z * cell_size - (MAP_HEIGHT as f32 * cell_size) / 2.0 + cell_size / 2.0;

    let mesh = meshes.add(Mesh::from(Capsule3d::default()));
    let material = materials.add(Color::srgb(0.2, 0.5, 1.0));

    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(world_x, CAMERA_HEIGHT, world_z),
            RigidBody::Dynamic,
            Collider::capsule_y(0.8, 0.5),
            LockedAxes::ROTATION_LOCKED,
            Player,
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 0.0, 0.0).looking_to(Vec3::X, Vec3::Y),
                PlayerCamera,
            ));
        });
}

// Find the first empty cell in a 4x4 box starting from a corner, going dx/dz direction
fn find_spawn_in_corner((x0, z0, dx, dz): (usize, usize, isize, isize)) -> (f32, f32) {
    let search = 4;
    for dz_i in 0..search {
        for dx_i in 0..search {
            let x = (x0 as isize + dx_i as isize * dx) as usize;
            let z = (z0 as isize + dz_i as isize * dz) as usize;
            if x < MAP_WIDTH && z < MAP_HEIGHT && MAZE[z][x] == 0 {
                return (x as f32, z as f32);
            }
        }
    }
    (x0 as f32, z0 as f32)
}

// WASD player movement (relative to facing direction)
fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut movement = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) { movement.x += 1.0; }
    if keyboard.pressed(KeyCode::KeyS) { movement.x -= 1.0; }
    if keyboard.pressed(KeyCode::KeyA) { movement.z += 1.0; }
    if keyboard.pressed(KeyCode::KeyD) { movement.z -= 1.0; }

    if movement.length_squared() > 0.0 {
        movement = movement.normalize() * PLAYER_SPEED * time.delta_secs();
        for mut transform in &mut query {
            let forward = transform.forward();
            let right = transform.right();
            let move_vec = forward * movement.z + right * movement.x;
            transform.translation += move_vec;
        }
    }
}

// FPS camera look with mouse (hold right mouse to look)
fn player_camera_look(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    if !mouse_buttons.pressed(MouseButton::Right) {
        return;
    }

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }
    if delta == Vec2::ZERO {
        return;
    }
    let sensitivity = MOUSE_SENSITIVITY;

    for mut player_transform in &mut player_query {
        // Yaw: rotate player horizontally
        player_transform.rotate_y(-delta.x * sensitivity * 0.01);
        for mut cam_transform in &mut camera_query {
            // Pitch: rotate camera locally
            let pitch_delta = -delta.y * sensitivity * 0.01;
            cam_transform.rotate_local_x(pitch_delta);
            // Clamp pitch (optional, ~±85deg)
            let rot = cam_transform.rotation.to_euler(EulerRot::YXZ);
            let pitch = rot.1.clamp(-1.5, 1.5);
            cam_transform.rotation = Quat::from_euler(EulerRot::YXZ, rot.0, pitch, rot.2);
        }
    }
}