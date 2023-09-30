pub mod components;
pub mod events;
pub mod styles;
mod systems;

use events::*;
use systems::*;

use bevy::prelude::*;

use self::components::ParticleSpawner;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ParticleSpawner>()
            .add_event::<SpawnParticlesEvent>()
            .add_systems(
                Update,
                (
                    on_spawn_particles_event,
                    emit_particles,
                    update_particle_lifetime,
                    update_particle_position.after(emit_particles),
                    update_particle_size.after(emit_particles),
                    update_particle_color.after(emit_particles),
                ),
            );
    }
}
