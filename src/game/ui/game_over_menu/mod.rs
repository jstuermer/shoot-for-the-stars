mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::AppState;

use systems::layout::{despawn_game_over_menu, spawn_game_over_menu};

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu);
        app.add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
