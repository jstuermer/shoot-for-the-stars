mod game_over_menu;
mod hud;
mod pause_menu;

use bevy::prelude::*;

use game_over_menu::GameOverMenuPlugin;
use hud::GameHUDPlugin;
use pause_menu::PauseMenuPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameHUDPlugin, GameOverMenuPlugin, PauseMenuPlugin));
    }
}
