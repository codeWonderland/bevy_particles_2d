use bevy::prelude::*;

pub fn get_particle_spawner_style() -> Style {
    Style {
        position_type: PositionType::Absolute,
        top: Val::Px(0.0),
        left: Val::Px(0.0),
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..Style::DEFAULT
    }
}

pub fn get_default_particle_style() -> Style {
    Style {
        position_type: PositionType::Absolute,
        ..Style::DEFAULT
    }
}