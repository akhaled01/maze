use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::{prelude::*, plugin::RapierContext};

use crate::{camera_controller::CameraController, player::Player, targets::{DeadTarget, Target}, tracer::BulletTracer};

// use super::{camera_controller::CameraController, player::Player};
// use crate::game::{
//     level::targets::{DeadTarget, Target},
//     shooting,
// };

#[derive(Component)]
pub struct Shootable;

#[derive(Component)]
pub struct TracerSpawnSpot;

pub fn update_player(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    rapier_context: RapierContext,
    mut player_query: Query<(
        &mut Player,
        &mut Transform,
    )>,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraController>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    target_query: Query<Option<&Target>, With<Shootable>>,
    spawn_spot: Query<&GlobalTransform, With<TracerSpawnSpot>>,
) {
    let spawn_spot = match spawn_spot.get_single() {
        Ok(s) => s,
        Err(_) => return,
    };
    let window = match window_query.get_single() {
        Ok(w) => w,
        Err(_) => return,
    };
    let (camera, camera_global_transform) = match camera_query.get_single() {
        Ok(v) => v,
        Err(_) => return,
    };
    if let Ok((_player, _transform)) = player_query.get_single_mut() {
        if mouse_input.just_pressed(MouseButton::Left) {
            let Ok(ray) = camera.viewport_to_world(
                &camera_global_transform,
                Vec2::new(window.width() / 2., window.height() / 2.),
            ) else {
                return;
            };
            let predicate = |handle| target_query.get(handle).is_ok();
            let query_filter = QueryFilter::new().predicate(&predicate);
            let hit = rapier_context.cast_ray_and_get_normal(
                ray.origin,
                ray.direction.into(),
                f32::MAX,
                true,
                query_filter,
            );
            if let Some((entity, ray_intersection)) = hit {
                if let Ok(target) = target_query.get(entity) {
                    if target.is_some() {
                        commands.entity(entity).insert(DeadTarget);
                    }
                }
                // spawn tracer and check collisions
                let tracer_material = StandardMaterial {
                    base_color: Color::srgb(1.0, 1.0, 0.0),
                    unlit: true,
                    ..default()
                };

                commands.spawn((
                    Transform::from_translation(Vec3::splat(f32::MAX)),
                    Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(0.1, 0.1, 1.0)))),
                    MeshMaterial3d(materials.add(tracer_material)),
                    BulletTracer::new(
                        spawn_spot.translation(),
                        ray_intersection.point,
                        300.,
                    ),
                ));
            }
        }
    }
}
