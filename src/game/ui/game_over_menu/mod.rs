mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::AppState;

use systems::{
    interactions::{
        interact_with_main_menu_button, interact_with_restart_button, reset_game, restart_game,
        transition_to_main_menu_state,
    },
    layout::{despawn_game_over_menu, spawn_game_over_menu},
};

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu);
        app.add_systems(
            Update,
            (
                interact_with_main_menu_button,
                interact_with_restart_button,
                transition_to_main_menu_state,
                reset_game,
                restart_game.after(reset_game),
                run_state_transitions.after(reset_game), // make sure that restarting makes other entities despawn
            )
                .run_if(in_state(AppState::GameOver)),
        );
        app.add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}

fn run_state_transitions(world: &mut World) {
    let _ = world.try_run_schedule(StateTransition);
}
