use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerInput {
    // x: forward/back, y: right/left
    pub movement: Vec3,
}

pub fn update_movement_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut input: ResMut<PlayerInput>,
) {
    input.movement = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        input.movement.x += 1.;
    }
    if keys.pressed(KeyCode::KeyA) {
        input.movement.z += 1.;
    }
    if keys.pressed(KeyCode::KeyS) {
        input.movement.x -= 1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        input.movement.z -= 1.;
    }
}
