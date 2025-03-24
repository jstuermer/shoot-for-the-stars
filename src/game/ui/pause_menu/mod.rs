mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use systems::layout::{despawn_pause_menu, spawn_pause_menu};

use crate::game::SimulationState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu);
        app.add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}
