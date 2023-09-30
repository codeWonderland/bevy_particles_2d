use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use super::events::SpawnParticlesEvent;
use super::styles::{get_default_particle_style, get_particle_spawner_style};

fn build_particle(
    spawner: &ParticleSpawner,
    window_query: &Query<&Window, With<PrimaryWindow>>,
) -> (
    Particle,
    NodeBundle,
    ParticleSize,
    ParticleColor,
    ParticleVelocity,
) {
    let window = window_query.get_single().unwrap();

    let particle = Particle {
        lifetime: Timer::from_seconds(spawner.particle_lifetime, TimerMode::Once),
    };

    let mut node = NodeBundle::default();
    node.visibility = Visibility::Hidden;

    node.style = get_default_particle_style();
    node.style.left = Val::Px(
        window.width() / 2.0 + spawner.position_variance * (2.0 * rand::random::<f32>() - 1.0),
    );
    node.style.top = Val::Px(spawner.position_variance * (2.0 * rand::random::<f32>() - 1.0));

    let size = spawner.particle_size.unwrap();
    node.style.width = Val::Px(size.start);
    node.style.height = Val::Px(size.start);

    let color = spawner.particle_color.unwrap();
    node.background_color = color.start.into();

    let mut velocity = spawner.particle_velocity.unwrap();
    velocity.start[0] = spawner.spray * (rand::random::<f32>() + spawner.offset);
    velocity.end[0] = spawner.spray * (rand::random::<f32>() + spawner.offset);

    (particle, node, size, color, velocity)
}

pub fn emit_particles(
    mut commands: Commands,
    mut spawners: Query<(
        Entity,
        &Children,
        &ParticleSpawner,
        &mut ParticleSpawnerTimer,
    )>,
    mut particles: Query<(&mut Particle, &mut Visibility, &mut Style)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    for (ent, children, spawner, mut timer) in spawners.iter_mut() {
        timer.spray_timer.tick(time.delta());
        timer.lifetime.tick(time.delta());

        if timer.lifetime.finished() {
            timer.finishing_time.tick(time.delta());

            if timer.finishing_time.just_finished() {
                commands.entity(ent).despawn_recursive();
            }
            continue;
        } else if timer.spray_timer.just_finished() {
            let window = window_query.get_single().unwrap();

            for _i in 0..spawner.amount_per_burst {
                for child in children.iter() {
                    if let Ok((mut particle, mut visibility, mut style)) = particles.get_mut(*child)
                    {
                        if *visibility == Visibility::Hidden {
                            particle.lifetime =
                                Timer::from_seconds(spawner.particle_lifetime, TimerMode::Once);
                            *visibility = Visibility::Visible;
                            style.left = Val::Px(
                                window.width() / 2.0
                                    + spawner.position_variance
                                        * (2.0 * rand::random::<f32>() - 1.0),
                            );
                            style.top = Val::Px(
                                spawner.position_variance * (2.0 * rand::random::<f32>() - 1.0),
                            );
                            break;
                        }
                    }
                }
            }
        }
    }
}

pub fn on_spawn_particles_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut spawn_particles_event_reader: EventReader<SpawnParticlesEvent>,
) {
    for _event in spawn_particles_event_reader.iter() {
        spawn_particle_spawner(&mut commands, &window_query);
        play_confetti_sound(&mut commands, &asset_server);
    }
}

fn spawn_particle_spawner(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
) {
    let ron_str = &std::fs::read_to_string("assets/basic_spawner.ron").unwrap();
    let spawner =
        ron::from_str::<ParticleSpawner>(ron_str).expect("Failed to load basic_spawner.ron");

    commands
        .spawn(NodeBundle {
            style: get_particle_spawner_style(),
            visibility: Visibility::Visible,
            ..default()
        })
        .insert(ParticleSpawnerTimer {
            spray_timer: Timer::from_seconds(spawner.rate, TimerMode::Repeating),
            lifetime: Timer::from_seconds(spawner.lifetime, TimerMode::Once),
            finishing_time: Timer::from_seconds(spawner.particle_lifetime, TimerMode::Once),
        })
        .insert(spawner)
        .with_children(|parent| {
            for _i in 0..((1.1 * spawner.particle_lifetime / spawner.rate).ceil() as usize
                * spawner.amount_per_burst)
            {
                parent.spawn(build_particle(&spawner, &window_query));
            }
        });
}

pub fn update_particle_lifetime(
    mut particles: Query<(&mut Particle, &mut Visibility)>,
    time: Res<Time>,
) {
    for (mut particle, mut visibility) in particles.iter_mut() {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.finished() {
            *visibility = Visibility::Hidden;
        }
    }
}

fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    a * (1.0 - t) + b * t
}

pub fn update_particle_position(
    mut particles: Query<(&Particle, &ParticleVelocity, &mut Style)>,
    time: Res<Time>,
) {
    for (particle, velocity, mut style) in particles.iter_mut() {
        let mut velocity = lerp_vec2(velocity.start, velocity.end, particle.lifetime.percent());
        velocity[1] += GRAVITY;
        let translation = (velocity * time.delta_seconds()).extend(0.0);

        style.left = Val::Px(style.left.evaluate(2.0).unwrap() + translation.x);
        style.top = Val::Px(style.top.evaluate(2.0).unwrap() + translation.y);
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub fn update_particle_size(mut particles: Query<(&Particle, &ParticleSize, &mut Style)>) {
    for (particle, size, mut style) in particles.iter_mut() {
        let size = lerp(size.start, size.end, particle.lifetime.percent());
        style.width = Val::Px(size);
        style.height = Val::Px(size);
    }
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::rgba(
        lerp(a.r(), b.r(), t),
        lerp(a.g(), b.g(), t),
        lerp(a.b(), b.b(), t),
        lerp(a.a(), b.a(), t),
    )
}

pub fn update_particle_color(
    mut particles: Query<(&Particle, &ParticleColor, &mut BackgroundColor)>,
) {
    for (particle, color, mut background) in particles.iter_mut() {
        let color = lerp_color(color.start, color.end, particle.lifetime.percent());
        *background = color.into();
    }
}

pub fn play_confetti_sound(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((AudioBundle {
        source: asset_server.load("audio/confetti.ogg"),
        settings: PlaybackSettings::ONCE.with_volume(Volume::new_relative(0.5)),
    },));
}
