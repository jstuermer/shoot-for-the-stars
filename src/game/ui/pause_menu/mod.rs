mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use systems::{
    interactions::{
        continue_game, interact_with_continue_button, interact_with_main_menu_button,
        interact_with_restart_button, reset_game, restart_game, transition_to_main_menu_state,
    },
    layout::{despawn_pause_menu, spawn_pause_menu},
};

use crate::{game::SimulationState, AppState};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SimulationState::Paused),
            spawn_pause_menu.run_if(in_state(AppState::Game)),
        );
        app.add_systems(
            Update,
            (
                interact_with_continue_button,
                interact_with_main_menu_button,
                interact_with_restart_button,
                transition_to_main_menu_state,
                continue_game,
                reset_game,
                restart_game.after(reset_game),
                run_state_transitions.after(reset_game), // make sure that restarting makes other entities despawn
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Paused)),
        );
        app.add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
        app.add_systems(OnExit(AppState::Game), despawn_pause_menu);
    }
}

fn run_state_transitions(world: &mut World) {
    let _ = world.try_run_schedule(StateTransition);
}
