use crate::state::{MAP_HEIGHT, MAP_WIDTH, Player};
use sdl2::keyboard::KeyboardState;

const MOVE_SPEED: f64 = 2.0; // units per second
const STRAFE_SPEED: f64 = 2.0; // units per second
const MOUSE_SENSITIVITY: f64 = 0.001; // radians per pixel

pub fn handle_input(
    keyboard: &KeyboardState,
    player: &mut Player,
    delta_time: f64,
    map: &[[u8; MAP_WIDTH]; MAP_HEIGHT],
    mouse_rel_x: i32,
) {
    use sdl2::keyboard::Scancode;

    // Forward/Backward movement
    if keyboard.is_scancode_pressed(Scancode::W) {
        let new_x = player.x + player.dir_x * MOVE_SPEED * delta_time;
        let new_y = player.y + player.dir_y * MOVE_SPEED * delta_time;
        if !wall_collision(new_x, new_y, map) {
            player.x = new_x;
            player.y = new_y;
        }
    }
    if keyboard.is_scancode_pressed(Scancode::S) {
        let new_x = player.x - player.dir_x * MOVE_SPEED * delta_time;
        let new_y = player.y - player.dir_y * MOVE_SPEED * delta_time;
        if !wall_collision(new_x, new_y, map) {
            player.x = new_x;
            player.y = new_y;
        }
    }

    // Strafe movement
    if keyboard.is_scancode_pressed(Scancode::A) {
        // Move perpendicular to the direction vector (left)
        let strafe_x = -player.dir_y * STRAFE_SPEED * delta_time;
        let strafe_y = player.dir_x * STRAFE_SPEED * delta_time;
        let new_x = player.x + strafe_x;
        let new_y = player.y + strafe_y;
        if !wall_collision(new_x, new_y, map) {
            player.x = new_x;
            player.y = new_y;
        }
    }
    if keyboard.is_scancode_pressed(Scancode::D) {
        // Move perpendicular to the direction vector (right)
        let strafe_x = player.dir_y * STRAFE_SPEED * delta_time;
        let strafe_y = -player.dir_x * STRAFE_SPEED * delta_time;
        let new_x = player.x + strafe_x;
        let new_y = player.y + strafe_y;
        if !wall_collision(new_x, new_y, map) {
            player.x = new_x;
            player.y = new_y;
        }
    }

    // Mouse rotation
    if mouse_rel_x != 0 {
        let rot_speed = mouse_rel_x as f64 * MOUSE_SENSITIVITY * -1.0;
        let old_dir_x = player.dir_x;
        player.dir_x = player.dir_x * f64::cos(rot_speed) - player.dir_y * f64::sin(rot_speed);
        player.dir_y = old_dir_x * f64::sin(rot_speed) + player.dir_y * f64::cos(rot_speed);
        let old_plane_x = player.plane_x;
        player.plane_x = player.plane_x * f64::cos(rot_speed) - player.plane_y * f64::sin(rot_speed);
        player.plane_y = old_plane_x * f64::sin(rot_speed) + player.plane_y * f64::cos(rot_speed);
    }
}

// Helper function to check for wall collisions
fn wall_collision(x: f64, y: f64, map: &[[u8; MAP_WIDTH]; MAP_HEIGHT]) -> bool {
    // Add a small collision buffer
    const COLLISION_BUFFER: f64 = 0.02;

    let check_positions = [
        (x + COLLISION_BUFFER, y + COLLISION_BUFFER),
        (x + COLLISION_BUFFER, y - COLLISION_BUFFER),
        (x - COLLISION_BUFFER, y + COLLISION_BUFFER),
        (x - COLLISION_BUFFER, y - COLLISION_BUFFER),
    ];

    for &(check_x, check_y) in check_positions.iter() {
        let map_x = check_x as usize;
        let map_y = check_y as usize;

        if map_x >= MAP_WIDTH || map_y >= MAP_HEIGHT || map[map_y][map_x] != 0 {
            return true;
        }
    }

    false
}
