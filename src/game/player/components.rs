use bevy::prelude::*;

use crate::game::components::{AnimationIndices, AnimationTimer, Health, Size, Velocity};

use super::{systems::PLAYER_SIZE, INITIAL_PLAYER_HEALTH};

#[derive(Component)]
#[require(
    Transform,
    Velocity,
    Size(player_size),
    Health(initial_player_health),
    Sprite,
    AnimationIndices(player_animation_indices),
    AnimationTimer(player_animation_timer)
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

fn player_animation_timer() -> AnimationTimer {
    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
}

fn player_animation_indices() -> AnimationIndices {
    AnimationIndices { first: 0, last: 2 }
}
