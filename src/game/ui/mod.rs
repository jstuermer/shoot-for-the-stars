mod game_over_menu;
mod hud;

use bevy::prelude::*;

use game_over_menu::GameOverMenuPlugin;
use hud::GameHUDPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameHUDPlugin, GameOverMenuPlugin));
    }
}
