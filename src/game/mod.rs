mod components;
mod enemy;
pub mod player;
mod score;
mod star;
mod systems;
mod ui;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use bevy::prelude::*;

use crate::{events::GameOver, AppState};

use self::{systems::toggle_simulation, ui::GameUIPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugins((
                EnemyPlugin,
                PlayerPlugin,
                ScorePlugin,
                StarPlugin,
                GameUIPlugin,
            ))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Paused,
    Running,
}
