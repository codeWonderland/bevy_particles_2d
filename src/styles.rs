use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::BLACK;

pub fn background_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..Style::DEFAULT
    }
}
