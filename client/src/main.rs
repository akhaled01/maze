use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use level::{spawn_ground, spawn_maze};

mod level;
mod players;
mod player;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Maze Fuckers".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: false,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(ClearColor(Color::srgb(0.18, 0.22, 0.35)))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_ground)
        .add_systems(Startup, spawn_maze)
        .add_systems(Startup, players::spawn_players);

    // Add player systems
    player::setup_player_systems(&mut app);

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
            illuminance: 40000.0,
            shadows_enabled: true,
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
}