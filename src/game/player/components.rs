use bevy::prelude::*;

use crate::game::components::{Health, Size, Velocity};

use super::{systems::PLAYER_SIZE, INITIAL_PLAYER_HEALTH};

#[derive(Component)]
#[require(
    Transform,
    Velocity,
    Size(player_size),
    Health(initial_player_health),
    Sprite
)]
pub struct Player;

pub fn player_size() -> Size {
    Size {
        width: PLAYER_SIZE,
        height: PLAYER_SIZE,
    }
}

fn initial_player_health() -> Health {
    Health {
        current: INITIAL_PLAYER_HEALTH,
    }
}
