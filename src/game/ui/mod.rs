mod hud;

use bevy::prelude::*;

use hud::GameHUDPlugin;
pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameHUDPlugin);
    }
}
