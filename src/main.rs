pub mod particles;
pub mod styles;
mod systems;

use particles::ParticlesPlugin;

use systems::*;

use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Particles Test".into(),
                mode: WindowMode::Fullscreen,
                ..default()
            }),
            ..default()
        }))
        //.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ParticlesPlugin)

        .add_systems(Startup, (
            spawn_background,
            spawn_camera
        ))
        .add_systems(Update, handle_input)
        .run()
        ;
}
