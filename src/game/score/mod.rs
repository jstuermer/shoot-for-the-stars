pub mod resources;
mod systems;

use bevy::prelude::*;

use resources::*;
use systems::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(
                OnExit(AppState::Game),
                (update_high_scores, remove_score).chain(),
            );
    }
}
