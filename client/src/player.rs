use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::math::primitives::{Capsule3d, Cuboid, Sphere};

use crate::level::{MAZE, MAP_WIDTH, MAP_HEIGHT};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletLifetime(pub f32); // seconds

const PLAYER_SPEED: f32 = 6.0;
const MOUSE_SENSITIVITY: f32 = 0.15;
const CAMERA_HEIGHT: f32 = 1.6;

pub fn setup_player_systems(app: &mut App) {
    app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (
            player_movement,
            player_camera_look,
            player_shooting_system,
            projectile_movement_system,
            bullet_lifetime_system,
        ));
}

// #[derive(Resource)]
// pub struct FireCooldown {
//     timer: Timer,
// }

// impl Default for FireCooldown {
//     fn default() -> Self {
//         Self { timer: Timer::from_seconds(0.1, TimerMode::Repeating) }
//     }
// }

#[derive(Component, Deref, DerefMut)]
pub struct WeaponCooldown(pub f32);

// How fast can you fire? (shots per second)
const FIRE_RATE: f32 = 10.0; // 10 shots/sec

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

    // Spawn player capsule
    let mesh = meshes.add(Mesh::from(Capsule3d::default()));
    let material = materials.add(Color::srgb(0.2, 0.5, 1.0));

    // Spawn weapon as child (simple box for now)
    let weapon_mesh = meshes.add(Mesh::from(Cuboid::new(0.2, 0.1, 0.5)));
    let weapon_material = materials.add(Color::srgb(0.3, 0.3, 0.3));

    commands
        .spawn((
            Player,
            WeaponCooldown(0.0),
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(world_x, CAMERA_HEIGHT, world_z),
            RigidBody::Dynamic,
            Collider::capsule_y(0.8, 0.5),
            LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y,
        ))
        .with_children(|parent| {
            // Enable below block of code to spawn a camera as child of player capsule (After commenting out Main Camera in main.rs)
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 0.0, 0.0).looking_to(Vec3::X, Vec3::Y),
                PlayerCamera,
            ));
            parent.spawn((
                Mesh3d(weapon_mesh),
                MeshMaterial3d(weapon_material),
                Transform::from_xyz(0.5, 0.5, 1.0),
                Name::new("Weapon"),
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

// fn shoot(
    // mut commands: Commands,
    // keyboard: Res<ButtonInput<MouseButton>>,
    // query: Query<(&Transform, &GlobalTransform), With<Player>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     if keyboard.just_pressed(MouseButton::Left) {
//         for (transform, global_transform) in &query {
//             // Spawn bullet
//             let bullet_dir = global_transform.forward();
//             let bullet_speed = 20.0;
//             commands.spawn((
//                 Mesh3d(meshes.add(Mesh::from(Sphere { radius: 0.05 }))),
//                 MeshMaterial3d(materials.add(Color::srgb(1.0, 0.8, 0.1))),
//                 Transform::from_translation(global_transform.translation() + bullet_dir * 1.2),
//                 RigidBody::Dynamic,
//                 Collider::ball(0.05),
//                 Velocity {
//                     linvel: bullet_dir * bullet_speed,
//                     angvel: Vec3::ZERO,
//                 },
//                 Name::new("Bullet"),
//             ));
//         }
//     }
// }

// pub fn shoot(
//     mut commands: Commands,
//     keyboard: Res<ButtonInput<MouseButton>>,
//     query: Query<(&Transform), With<Player>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // Only handle one player for now
//     if let Ok(player_transform) = query.get_single() {
//         // Spawn the projectile at player position, moving in their forward direction
//         let forward = player_transform.forward(); // Vec3
//         let spawn_position = player_transform.translation + forward * 1.0; // Slightly ahead of player

//         commands.spawn((
//             Mesh3d(meshes.add(Mesh::from(Sphere { radius: 0.05 }))),
//             MeshMaterial3d(materials.add(Color::srgb(1.0, 0.8, 0.1))),
//             Transform::from_translation(spawn_position),
//             Bullet,
//             Velocity(forward * 20.0), // You'll need a velocity component for projectile movement
//         ));
//     }
// }

// pub fn player_shooting_system(
//     mut commands: Commands,
//     time: Res<Time>,
//     mouse_button_input: Res<ButtonInput<MouseButton>>,
//     mut query: Query<(&mut Player, &GlobalTransform)>,
// ) {
//     for (mut player, global_transform) in query.iter_mut() {
//         // Update the timer
//         player.fire_timer.tick(time.delta());

//         // Only allow shooting if timer finished and mouse is held
//         if mouse_button_input.pressed(MouseButton::Left) && player.fire_timer.finished() {
//             // Spawn projectile at camera/player position, going forward
//             let transform = global_transform.compute_transform();
//             let forward = transform.forward();
//             let position = transform.translation + forward * 1.0; // slightly ahead of player

//             commands.spawn((
//                 Bullet,
//                 Transform::from_translation(position).looking_to(forward, Vec3::Y),
//                 // Add velocity as a component, etc
//             ));

//             // Reset the timer
//             player.fire_timer.reset();
//         }
//     }
// }

// pub fn player_shooting_system(
//     mut commands: Commands,
//     buttons: Res<ButtonInput<MouseButton>>,
//     time: Res<Time>,
//     mut cooldown: ResMut<FireCooldown>,
//     query: Query<(&Transform, &GlobalTransform), With<PlayerCamera>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // Allow continuous shooting while LMB held down, and respect cooldown.
//     cooldown.timer.tick(time.delta());
//     if buttons.pressed(MouseButton::Left) && cooldown.timer.finished() {
//         // Find player camera position & orientation
//         if let Ok((transform, global_transform)) = query.get_single() {
//             let spawn_pos = global_transform.translation();

//             // "Forward" direction in Bevy is negative Z
//             let forward = global_transform.forward();

//             // Spawn the projectile (simple mesh or your projectile scene)
//             commands.spawn((
//                 Mesh3d(meshes.add(Mesh::from(Sphere { radius: 0.05 }))),
//                 MeshMaterial3d(materials.add(Color::srgb(1.0, 0.8, 0.1))),
//                 Bullet,
//                 // Give it a velocity using Rapier
//                 RigidBody::Dynamic,
//                 Collider::ball(0.1), // Sphere collider as example
//                 Velocity::linear(forward * 30.0), // Fast projectile
//                 // Add despawn timer or similar if you want
//             ));

//             // Reset the cooldown timer
//             cooldown.timer.reset();
//         }
//     }
// }

// SYSTEM: Handles rapid fire when holding left mouse
pub fn player_shooting_system(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut WeaponCooldown, &GlobalTransform), With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut cooldown, global_transform) in &mut query {
        // Decrease cooldown
        **cooldown -= time.delta_secs();

        if mouse.pressed(MouseButton::Left) && **cooldown <= 0.0 {
            // Ready to shoot!
            **cooldown = 1.0 / FIRE_RATE;

            let spawn_pos = global_transform.translation() + Vec3::new(-3.0, -1.0, 0.0);
            let forward = global_transform.forward();
            let right = global_transform.right();

            // Spawn the projectile
            commands.spawn((
                Bullet,
                Transform::from_translation(spawn_pos),
                Velocity(right * 30.0), // Your projectile speed here
                Mesh3d(meshes.add(Mesh::from(Sphere { radius: 0.50 }))),
                MeshMaterial3d(materials.add(Color::srgb(1.0, 0.8, 0.1))),
                // Rigid body: kinematic or dynamic
                RigidBody::KinematicVelocityBased,
                Collider::ball(0.5),
                BulletLifetime(3.0),
            ));

            print!("Shoot bullet\n");

            // Optionally, add mesh/render component here
        }
    }
}

fn bullet_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut BulletLifetime)>
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.0 -= time.delta_secs();
        if lifetime.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

// SYSTEM: Moves projectiles each frame
pub fn projectile_movement_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<Bullet>>,
) {
    for (velocity, mut transform) in &mut query {
        transform.translation += **velocity * time.delta_secs();
    }
}