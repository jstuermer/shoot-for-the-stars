pub mod components;
mod systems;

use bevy::prelude::*;

use systems::*;

use super::SimulationState;
use crate::AppState;

pub const INITIAL_PLAYER_HEALTH: u32 = 3;
pub const PLAYER_SPRITE: &str = "sprites/spacecraft.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (
                    animate_player_sprite,
                    player_movement,
                    confine_player_movement.after(player_movement),
                    player_hit_star,
                    (player_hit_enemy, check_player_health, handle_game_over).chain(),
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
