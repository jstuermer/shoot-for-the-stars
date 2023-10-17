mod enemy;
mod player;
mod score;
mod star;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use crate::events::GameOver;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>().add_plugins((
            EnemyPlugin,
            PlayerPlugin,
            ScorePlugin,
            StarPlugin,
        ));
    }
}
