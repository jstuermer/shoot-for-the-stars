use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use super::{
    components::{star_size, Star},
    resources::StarSpawnTimer,
    NUMBER_OF_STARS,
};
use crate::utils;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let size = star_size();
    let [x_min, x_max, y_min, y_max] = utils::get_confinement(window, size.width, size.height);
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for _ in 0..NUMBER_OF_STARS {
        let x_position: f32 = rng.gen_range(x_min..=x_max);
        let y_position: f32 = rng.gen_range(y_min..=y_max);

        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/star.png")),
            Transform::from_xyz(x_position, y_position, 0.0),
            Star,
        ));
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if !star_spawn_timer.timer.finished() {
        return;
    }

    let window: &Window = window_query.get_single().unwrap();
    let size = star_size();
    let [x_min, x_max, y_min, y_max] = utils::get_confinement(window, size.width, size.height);
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let x_position: f32 = rng.gen_range(x_min..=x_max);
    let y_position: f32 = rng.gen_range(y_min..=y_max);

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/star.png")),
        Transform::from_xyz(x_position, y_position, 0.0),
        Star,
    ));
}

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for star_entity in &star_query {
        commands.entity(star_entity).despawn();
    }
}
