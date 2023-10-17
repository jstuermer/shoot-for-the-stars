pub mod components;
mod events;
mod game;
mod main_menu;
mod systems;
mod utils;

use bevy::prelude::*;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((MainMenuPlugin, GamePlugin))
        .add_systems(Startup, spawn_camera)
        .run();
}
