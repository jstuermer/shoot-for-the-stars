pub mod components;
mod resources;
mod systems;

use bevy::prelude::*;

use resources::*;
use systems::*;

pub const ENEMY_SIZE: f32 = 64.0; // this is the size of the enemy sprite
const ENEMY_SPEED: f32 = 200.0;
const NUMBER_OF_ENEMIES: usize = 4;
const ENEMY_TIMESTEP: f32 = 1.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .insert_resource(FixedTime::new_from_secs(ENEMY_TIMESTEP))
            .add_systems(Startup, spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    confine_enemy_movement.after(enemy_movement),
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                ),
            )
            .add_systems(
                FixedUpdate,
                enemy_redirection.before(confine_enemy_movement),
            );
    }
}
