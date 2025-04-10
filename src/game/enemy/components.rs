use bevy::prelude::*;

use crate::game::components::{Size, Velocity};

use super::ENEMY_SIZE;

#[derive(Component)]
#[require(Transform, Velocity, Size(enemy_size), Sprite)]
pub struct Enemy;

pub fn enemy_size() -> Size {
    Size {
        width: ENEMY_SIZE,
        height: ENEMY_SIZE,
    }
}
