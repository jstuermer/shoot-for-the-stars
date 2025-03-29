pub mod components;
mod resources;
mod systems;

use bevy::prelude::*;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

/// Size of the enemy sprite in pixels.
pub const ENEMY_SIZE: f32 = 64.0;
pub const INITIAL_NUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SPEED: f32 = 200.0;
const ENEMY_TIMESTEP: f64 = 1.0;
pub const ENEMY_SPRITE: &str = "sprites/asteroid.png";

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .insert_resource(Time::from_seconds(ENEMY_TIMESTEP))
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    confine_enemy_movement.after(enemy_movement),
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                FixedUpdate,
                enemy_redirection
                    .before(confine_enemy_movement)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}
