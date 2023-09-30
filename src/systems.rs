use bevy::{prelude::*, window::PrimaryWindow, app::AppExit};

use crate::{particles::events::SpawnParticlesEvent, styles::{background_style, BACKGROUND_COLOR}};

pub fn spawn_camera(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands
        .spawn(
            Camera2dBundle {
                // transform: Transform::from_xyz(0.0, 0.0, 0.0),
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                ..default()
            }
        );
}

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        ImageBundle {
            style: background_style(),
            background_color: BACKGROUND_COLOR.into(),
            ..default()
        }
    );
}

pub fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut spawn_particles_event_writer: EventWriter<SpawnParticlesEvent>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        spawn_particles_event_writer.send(SpawnParticlesEvent {});
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}
