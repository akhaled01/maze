// use bevy::prelude::*;
// use bevy_rapier3d::prelude::*;
// use bevy_fps_controller::controller::*;
// use bevy::input::mouse::MouseMotion;
// use bevy::math::primitives::{Capsule3d, Cuboid, Sphere};

// use crate::level::{MAZE, MAP_WIDTH, MAP_HEIGHT};

// /// Marker for the logical player entity.
// #[derive(Component)]
// pub struct LogicalPlayer;

// /// Marker for the rendering camera attached to the player.
// #[derive(Component)]
// pub struct RenderPlayer {
//     pub logical_entity: Entity,
// }

// /// Additional camera config for FpsController
// #[derive(Component)]
// pub struct CameraConfig {
//     pub height_offset: f32,
// }

// #[derive(Component)]
// pub struct Player;

// #[derive(Component)]
// pub struct PlayerCamera;

// #[derive(Component)]
// pub struct Bullet;

// #[derive(Component)]
// pub struct BulletLifetime(pub f32); // seconds

// #[derive(Resource, Default)]
// pub struct MouseLookState {
//     pub yaw: f32,   // Horizontal angle, around Y
//     pub pitch: f32, // Vertical angle, around X (clamped)
// }

// const PLAYER_SPEED: f32 = 6.0;
// pub const MOUSE_SENSITIVITY: f32 = 0.002;
// pub const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.05; // ~ +-89 degrees
// const CAMERA_HEIGHT: f32 = 1.6;

// pub fn setup_player_systems(app: &mut App) {
//     app
//         .add_systems(Startup, spawn_player)
//         .add_systems(Update, (
//             player_movement,
//             player_look_mouse,
//             player_shooting_system,
//             projectile_movement_system,
//             bullet_lifetime_system,
//         ));
// }

// #[derive(Component, Deref, DerefMut)]
// pub struct WeaponCooldown(pub f32);

// // How fast can you fire? (shots per second)
// const FIRE_RATE: f32 = 10.0; // 10 shots/sec

// /// Spawns the player, camera, and sets up the FPS controller.
// pub fn spawn_player(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let cell_size = 2.0;
//     // Find first available corner (top-left) empty cell
//     let (spawn_x, spawn_z) = find_spawn_in_corner((0, 0, 1, 1));
//     let world_x = spawn_x * cell_size - (MAP_WIDTH as f32 * cell_size) / 2.0 + cell_size / 2.0;
//     let world_z = spawn_z * cell_size - (MAP_HEIGHT as f32 * cell_size) / 2.0 + cell_size / 2.0;

//     // Spawn player capsule
//     let mesh = meshes.add(Mesh::from(Capsule3d::default()));
//     let material = materials.add(Color::srgb(0.2, 0.5, 1.0));

//     // Spawn weapon as child (simple box for now)
//     let weapon_mesh = meshes.add(Mesh::from(Cuboid::new(0.2, 0.1, 0.5)));
//     let weapon_material = materials.add(Color::srgb(0.3, 0.3, 0.3));

//     commands
//         .spawn((
//             Player,
//             WeaponCooldown(0.0),
//             Mesh3d(mesh),
//             MeshMaterial3d(material),
//             Transform::from_xyz(world_x, CAMERA_HEIGHT, world_z),
//             RigidBody::KinematicPositionBased,
//             Collider::capsule_y(0.8, 0.5),
//             LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y,
//         ))
//         .with_children(|parent| {
//             // Enable below block of code to spawn a camera as child of player capsule (After commenting out Main Camera in main.rs)
//             parent.spawn((
//                 Camera3d::default(),
//                 Transform::from_xyz(0.0, 0.0, 0.0).looking_to(Vec3::X, Vec3::Y),
//             )).insert(PlayerCamera);
//             parent.spawn((
//                 Mesh3d(weapon_mesh),
//                 MeshMaterial3d(weapon_material),
//                 Transform::from_xyz(0.5, 0.5, 1.0),
//                 Name::new("Weapon"),
//             ));
//         });
// }

// // Find the first empty cell in a 4x4 box starting from a corner, going dx/dz direction
// fn find_spawn_in_corner((x0, z0, dx, dz): (usize, usize, isize, isize)) -> (f32, f32) {
//     let search = 4;
//     for dz_i in 0..search {
//         for dx_i in 0..search {
//             let x = (x0 as isize + dx_i as isize * dx) as usize;
//             let z = (z0 as isize + dz_i as isize * dz) as usize;
//             if x < MAP_WIDTH && z < MAP_HEIGHT && MAZE[z][x] == 0 {
//                 return (x as f32, z as f32);
//             }
//         }
//     }
//     (x0 as f32, z0 as f32)
// }

// // WASD player movement (relative to facing direction)
// fn player_movement(
//     time: Res<Time>,
//     keyboard: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut Transform, With<Player>>,
// ) {
//     let mut movement = Vec3::ZERO;
//     if keyboard.pressed(KeyCode::KeyW) { movement.x += 1.0; }
//     if keyboard.pressed(KeyCode::KeyS) { movement.x -= 1.0; }
//     if keyboard.pressed(KeyCode::KeyA) { movement.z += 1.0; }
//     if keyboard.pressed(KeyCode::KeyD) { movement.z -= 1.0; }

//     if movement.length_squared() > 0.0 {
//         movement = movement.normalize() * PLAYER_SPEED * time.delta_secs();
//         for mut transform in &mut query {
//             let forward = transform.forward();
//             let right = transform.right();
//             let move_vec = forward * movement.z + right * movement.x;
//             transform.translation += move_vec;
//         }
//     }
// }

// pub fn player_look_mouse(
//     mut mouse_events: EventReader<bevy::input::mouse::MouseMotion>,
//     mut state: ResMut<MouseLookState>,
//     mouse_button: Res<ButtonInput<MouseButton>>,
//     windows: Query<&Window>,
// ) {
//     // Only rotate if left mouse is pressed or window is focused
//     if !mouse_button.pressed(MouseButton::Left) {
//         return;
//     }

//     let mut delta = Vec2::ZERO;
//     for ev in mouse_events.read() {
//         delta += ev.delta;
//     }

//     state.yaw   -= delta.x * MOUSE_SENSITIVITY;
//     state.pitch -= delta.y * MOUSE_SENSITIVITY;

//     // Clamp pitch so you can't flip the camera upside down
//     state.pitch = state.pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);
// }

// // SYSTEM: Handles rapid fire when holding left mouse
// pub fn player_shooting_system(
//     time: Res<Time>,
//     mouse: Res<ButtonInput<MouseButton>>,
//     mut query: Query<(&mut WeaponCooldown, &GlobalTransform), With<Player>>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     for (mut cooldown, global_transform) in &mut query {
//         // Decrease cooldown
//         **cooldown -= time.delta_secs();

//         if mouse.pressed(MouseButton::Left) && **cooldown <= 0.0 {
//             // Ready to shoot!
//             **cooldown = 1.0 / FIRE_RATE;

//             let spawn_pos = global_transform.translation() + Vec3::new(-3.0, -1.0, 0.0);
//             let forward = global_transform.forward();
//             let right = global_transform.right();

//             // Spawn the projectile
//             commands.spawn((
//                 Bullet,
//                 Transform::from_translation(spawn_pos),
//                 Velocity(right * 30.0), // Your projectile speed here
//                 Mesh3d(meshes.add(Mesh::from(Sphere { radius: 0.020 }))),
//                 MeshMaterial3d(materials.add(Color::srgb(1.0, 0.8, 0.1))),
//                 // Rigid body: kinematic or dynamic
//                 RigidBody::KinematicVelocityBased,
//                 Collider::ball(0.5),
//                 BulletLifetime(3.0),
//             ));

//             print!("Shoot bullet\n");

//             // Optionally, add mesh/render component here
//         }
//     }
// }

// fn bullet_lifetime_system(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut query: Query<(Entity, &mut BulletLifetime)>
// ) {
//     for (entity, mut lifetime) in &mut query {
//         lifetime.0 -= time.delta_secs();
//         if lifetime.0 <= 0.0 {
//             commands.entity(entity).despawn();
//         }
//     }
// }

// #[derive(Component, Deref, DerefMut)]
// pub struct Velocity(pub Vec3);

// // SYSTEM: Moves projectiles each frame
// pub fn projectile_movement_system(
//     time: Res<Time>,
//     mut query: Query<(&Velocity, &mut Transform), With<Bullet>>,
// ) {
//     for (velocity, mut transform) in &mut query {
//         transform.translation += **velocity * time.delta_secs();
//     }
// }


use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    camera_controller::{update_camera_controller, CameraController},
    input::{update_movement_input, PlayerInput},
    player_movement::update_movement,
    player_shooting::{update_player, TracerSpawnSpot},
    tracer::TracerPlugin,
    utils::blender_to_world
};

// use super::{
//     camera_controller::{self, CameraController},
//     input::*,
//     player_movement::*,
//     player_shooting::{update_player, TracerSpawnSpot},
// };
// use crate::game::{math::coordinates::blender_to_world, shooting};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TracerPlugin)
            .init_resource::<PlayerInput>()
            .add_systems(Startup, init_player)
            .add_systems(FixedUpdate, update_movement)
            .add_systems(Update, update_movement_input)
            .add_systems(Update, update_camera_controller)
            .add_systems(Update, update_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub gravity: f32,
    pub speed: f32,
}

fn init_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fov = 103.0_f32.to_radians();
    const CAMERA_HEIGHT: f32 = 1.6;

    // Camera
    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 0.0, 0.0).looking_to(Vec3::X, Vec3::Y),
            CameraController {
                sensitivity: 0.035,
                rotation: Vec2::ZERO,
                rotation_lock: 88.0,
            },
        ))
        .id();

    // Gun model
    let gun_model = asset_server.load("models/ak.glb#Scene0");
    // let gun_entity = commands
    //     .spawn((
    //         SceneRoot(gun_model),
    //         Transform::IDENTITY,
    //     ))
    //     .id();

    // Spawn spot for tracer
    let spawn_spot = blender_to_world(Vec3::new(0.530462, 2.10557, -0.466568));
    // let tracer_spawn_entity = commands
    //     .spawn((
    //         Transform::from_translation(spawn_spot),
    //         // GlobalTransform::default(),
    //         TracerSpawnSpot,
    //     ))
    //     .id();

    // Player
    let player_entity = commands
        .spawn((
            Player {
                velocity: Vec3::ZERO,
                gravity: 9.8,
                speed: 20.0,
            },
            Transform::from_translation(Vec3::new(0., CAMERA_HEIGHT, 0.)),
            Visibility::default(),
            Collider::cuboid(1., 10., 1.),
            RigidBody::KinematicPositionBased,
            KinematicCharacterController {
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                ..default()
            },
        ))
        .id();

    commands.entity(camera_entity).with_children(|parent| {
        parent.spawn((
            SceneRoot(gun_model),
            Transform::IDENTITY,
            Name::new("Gun"),
        )).id();
        parent.spawn((
            Transform::from_translation(spawn_spot),
            TracerSpawnSpot,
            Name::new("Tracer Spawn Spot"),
        )).id();
    });
    commands.entity(player_entity).add_child(camera_entity);
}
