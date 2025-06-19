use bevy::{input::mouse::MouseMotion, prelude::*};

// #[derive(Component)]
// pub struct CameraController {
//     pub rotation: Vec2,
//     pub rotation_lock: f32,
//     pub sensitivity: f32,
// }

#[derive(Component)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
    pub pitch_limit: f32,
    pub enabled: bool,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.0015,
            pitch_limit: std::f32::consts::FRAC_PI_2 - 0.1,
            enabled: true,
        }
    }
}

pub fn update_camera_controller(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &mut CameraController)>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        for (mut transform, mut controller) in query.iter_mut() {
            let mut delta = Vec2::ZERO;
            for event in mouse_motion_events.read() {
                delta += event.delta;
            }

            if controller.enabled {
                controller.yaw -= delta.x * controller.sensitivity;
                controller.pitch -= delta.y * controller.sensitivity;

                controller.pitch = controller.pitch.clamp(
                    -controller.pitch_limit,
                    controller.pitch_limit,
                );

                let yaw_rotation = Quat::from_axis_angle(Vec3::Y, controller.yaw);
                let pitch_rotation = Quat::from_axis_angle(Vec3::X, controller.pitch);

                transform.rotation = yaw_rotation * pitch_rotation;
            }
        }
    }
}

// pub fn update_camera_controller(
//     mut mouse_motion: EventReader<MouseMotion>,
//     mut camera_query: Query<(&mut CameraController, &mut Transform)>,
// ) {
//     let pitch_limit = std::f32::consts::FRAC_PI_2 - 0.1;
//     if let Ok((mut camera_controller, mut transform)) = camera_query.get_single_mut() {
//         for ev in mouse_motion.read() {
//             camera_controller.rotation.y -= ev.delta.x * camera_controller.sensitivity;
//             camera_controller.rotation.x -= ev.delta.y * camera_controller.sensitivity;

//             camera_controller.rotation.x = camera_controller
//                 .rotation
//                 .x
//                 .clamp(-camera_controller.rotation_lock, camera_controller.rotation_lock);
//         }
//         let y_quat = Quat::from_axis_angle(Vec3::Y, camera_controller.rotation.y.to_radians());
//         let x_quat = Quat::from_axis_angle(Vec3::X, camera_controller.rotation.x.to_radians());
//         transform.rotation = y_quat * x_quat;
//     }
// }
