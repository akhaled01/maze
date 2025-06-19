use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::camera_controller::CameraController;
use crate::player::Player;

// use super::{camera_controller::CameraController, input::PlayerInput, player::Player};

#[derive(Component)]
pub struct PlayerMovement {
    pub speed: f32,
    pub jump_strength: f32,
    pub is_jumping: bool,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            speed: 5.0,
            jump_strength: 5.0,
            is_jumping: false,
        }
    }
}

pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &PlayerMovement), With<Player>>,
    camera_query: Query<(&Transform, &CameraController), (With<CameraController>, Without<Player>)>,
) {
    let (camera_transform, camera_controller) = match camera_query.get_single() {
        Ok(res) => res,
        Err(_) => return,
    };

    for (mut transform, movement) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        // Movement input
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += *camera_transform.forward();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= *camera_transform.forward();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= *camera_transform.right();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += *camera_transform.right();
        }

        direction.y = 0.0; // Ignore vertical direction
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        // Move player
        transform.translation += direction * movement.speed * time.delta_secs();
    }
}

// pub fn update_movement(
//     time: Res<Time<Fixed>>,
//     input: Res<PlayerInput>,
//     camera_query: Query<&CameraController>,
//     mut player_query: Query<(
//         &mut Player,
//         &mut KinematicCharacterController,
//         Option<&KinematicCharacterControllerOutput>
//     )>,
// ) {
//     let camera = camera_query.get_single().unwrap();

//     for (mut player, mut controller, controller_output) in player_query.iter_mut() {
//         if let Some(output) = controller_output {
//             if output.grounded {
//                 player.velocity = Vec3::ZERO;
//             }
//         }
//         let camera_rotation_converted = -camera.rotation.y.to_radians() - 90.0_f32.to_radians();

//         let forward = Vec2::new(
//             f32::cos(camera_rotation_converted),
//             f32::sin(camera_rotation_converted),
//         );

//         let right = Vec2::new(-forward.y, forward.x);

//         if let Some(movement_direction) =
//             (forward * input.movement.x + right * input.movement.y).try_normalize()
//         {
//             player.velocity.x = movement_direction.x * player.speed;
//             player.velocity.z = movement_direction.y * player.speed;
//         }
//         player.velocity.y -= player.gravity * time.timestep().as_secs_f32();

//         controller.translation = Some(player.velocity * time.timestep().as_secs_f32());
//     }
// }
