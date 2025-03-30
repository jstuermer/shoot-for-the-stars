use bevy::prelude::*;

/// Health component that determines whether an entity lives or dies.
/// An entity with a current health of 0 is considered dead.
#[derive(Component, Debug)]
pub struct Health {
    pub current: u32,
}

/// Velocity vector that makes an entity move.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Velocity {
    pub current: Vec3,
}
