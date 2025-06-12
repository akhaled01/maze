use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

// Set up UI elements
pub fn setup_ui(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_systems(Startup, init_fps_counter);
    app.add_systems(Update, update_fps_text);
}

// Init the FPS counter
fn init_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("FPS: ..."),
            TextFont {
                font: asset_server.load("fonts/PoetsenOne-Regular.ttf"),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
            FpsText,
        Name::new("FPS Counter Root"),
    ));
}

// Update the FPS text
fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    if let Some(fps) = diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            for mut text in &mut query {
                text.0 = format!("FPS: {:.0}", average);
            }
        }
    }
}


#[derive(Component)]
struct FpsText;
