use bevy::prelude::*;
use serde::Deserialize;

pub const GRAVITY: f32 = 30.0;

#[derive(Component, Clone, Copy, Deserialize, Reflect)]
pub struct ParticleSize {
    pub start: f32,
    pub end: f32,
}

#[derive(Component, Clone, Copy, Deserialize, Reflect)]
pub struct ParticleColor {
    pub start: Color,
    pub end: Color,
}

#[derive(Component, Clone, Copy, Deserialize, Reflect)]
pub struct ParticleVelocity {
    pub start: Vec2,
    pub end: Vec2,
}

#[derive(Component)]
pub struct ParticleSpawnerTimer {
    pub spray_timer: Timer,
    pub lifetime: Timer,
    pub finishing_time: Timer,
}

#[derive(Component, Clone, Copy, Deserialize, Reflect)]
pub struct ParticleSpawner {
    pub rate: f32,
    pub amount_per_burst: usize,
    pub position_variance: f32,
    pub spray: f32,
    pub offset: f32,
    pub lifetime: f32,
    pub particle_lifetime: f32,
    pub particle_size: Option<ParticleSize>,
    pub particle_velocity: Option<ParticleVelocity>,
    pub particle_color: Option<ParticleColor>,
}

#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
}

#[derive(Component)]
pub struct ParticleCollection {
    pub entities: Vec<Entity>,
}
