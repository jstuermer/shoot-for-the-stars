pub mod components;
mod enemy;
mod events;
mod player;
mod score;
mod star;
mod systems;
mod utils;

use bevy::prelude::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use events::*;
use systems::*;

fn main() {
    App::new()
        .add_event::<GameOver>()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            EnemyPlugin,
            ScorePlugin,
            StarPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}
