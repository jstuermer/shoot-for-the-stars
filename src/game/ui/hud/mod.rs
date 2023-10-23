mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::game::SimulationState;
use crate::AppState;
use systems::layout::spawn_game_hud;
use systems::updates::*;

pub struct GameHUDPlugin;

impl Plugin for GameHUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_game_hud);
        app.add_systems(
            Update,
            (
                update_enemy_number_info,
                update_health_info,
                update_score_info,
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(OnExit(AppState::Game), despawn_game_hud);
    }
}
