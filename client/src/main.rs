use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use level::{spawn_ground, spawn_walls};
use ui::setup_ui;

use crate::{input::PlayerInput, player::PlayerPlugin, tracer::TracerPlugin};

//use crate::player::FireCooldown;

mod level;
mod players;
mod player;
mod camera_controller;
mod input;
mod player_movement;
mod player_shooting;
mod tracer;
mod targets;
mod ui;
mod utils;

fn main() {
    let mut app = App::new();

    app
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Maze Fuckers".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: false,
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            PlayerPlugin,
        ))
        // .init_resource::<FireCooldown>()
        .insert_resource(TimestepMode::Fixed {
            dt: 1.0 / 60.0,
            substeps: 1,
        })
        // .insert_resource(PlayerInput::default())
        .insert_resource(ClearColor(Color::srgb(0.18, 0.22, 0.35)))
        .add_systems(Startup, (
            setup,
            spawn_ground,
            spawn_walls,
            players::spawn_players,
        ));

    // Add player systems
    // player::setup_player_systems(&mut app);

    // Add UI system
    ui::setup_ui(&mut app);

    app.run();
}

fn setup(mut commands: Commands) {
    //Camera Settings
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(0.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     GlobalTransform::default(),
    // ));

    //Light Settings
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 15.0, 0.0),
            rotation: Quat::from_euler(EulerRot::XYZ, -std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4, 0.0),
            ..default()
        },
        GlobalTransform::default(),
        bevy::pbr::NotShadowCaster,
    ));

    commands.spawn((
        PointLight {
            intensity: 5000.0,
            shadows_enabled: false,
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            ..default()
        }
    ));
}