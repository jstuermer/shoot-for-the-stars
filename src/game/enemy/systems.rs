use bevy::{prelude::*, window::PrimaryWindow};
use rand::seq::SliceRandom;
use rand::Rng;

use super::components::*;
use super::resources::*;
use super::{ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES};
use crate::utils;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let [x_min, x_max, y_min, y_max] = utils::get_confinement(window, ENEMY_SIZE);
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for _ in 0..NUMBER_OF_ENEMIES {
        let x_position: f32 = rng.gen_range(x_min..=x_max);
        let y_position: f32 = rng.gen_range(y_min..=y_max);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x_position, y_position, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec3::ZERO,
            },
        ));
    }
}

pub fn enemy_redirection(mut enemy_query: Query<&mut Enemy>) {
    let sample_directions: [f32; 3] = [-1.0, 0.0, 1.0];
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for mut enemy in &mut enemy_query {
        let mut direction = Vec3::ZERO;
        let x_random: &f32 = sample_directions
            .choose(&mut rng)
            .expect("Random x direction should have been generated.");
        let y_random: &f32 = sample_directions
            .choose(&mut rng)
            .expect("Random y direction should have been generated.");
        direction += Vec3::new(*x_random, *y_random, 0.0);
        enemy.direction = direction.normalize_or_zero();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut enemy_transform, enemy) in &mut enemy_query {
        enemy_transform.translation += enemy.direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn confine_enemy_movement(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let [x_min, x_max, y_min, y_max] = utils::get_confinement(window, ENEMY_SIZE);

    for (mut enemy_transform, mut enemy) in &mut enemy_query {
        let mut changed_direction: bool = false;

        if enemy_transform.translation.x < x_min {
            enemy_transform.translation.x = x_min;
            enemy.direction.x = -enemy.direction.x;
            changed_direction = true;
        } else if enemy_transform.translation.x > x_max {
            enemy_transform.translation.x = x_max;
            enemy.direction.x = -enemy.direction.x;
            changed_direction = true;
        }
        if enemy_transform.translation.y < y_min {
            enemy_transform.translation.y = y_min;
            enemy.direction.y = -enemy.direction.y;
            changed_direction = true;
        } else if enemy_transform.translation.y > y_max {
            enemy_transform.translation.y = y_max;
            enemy.direction.y = -enemy.direction.y;
            changed_direction = true;
        }

        if changed_direction {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/pluck_001.ogg"),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if !enemy_spawn_timer.timer.finished() {
        return;
    }

    let window: &Window = window_query.get_single().unwrap();
    let [x_min, x_max, y_min, y_max] = utils::get_confinement(window, ENEMY_SIZE);
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let x_position: f32 = rng.gen_range(x_min..=x_max);
    let y_position: f32 = rng.gen_range(y_min..=y_max);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x_position, y_position, 0.0),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        },
        Enemy {
            direction: Vec3::ZERO,
        },
    ));
}
