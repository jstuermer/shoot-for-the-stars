use bevy::prelude::*;

use crate::game::components::Size;

use super::STAR_SIZE;

#[derive(Component)]
#[require(Transform, Sprite, Size(star_size))]
pub struct Star;

pub fn star_size() -> Size {
    Size {
        width: STAR_SIZE,
        height: STAR_SIZE,
    }
}
