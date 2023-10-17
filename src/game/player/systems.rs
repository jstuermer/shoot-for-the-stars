use bevy::{prelude::*, window::PrimaryWindow};

use super::components::*;
use crate::components::Health;
use crate::events::GameOver;
use crate::game::enemy::components::Enemy;
use crate::game::enemy::ENEMY_SIZE;
use crate::game::score::resources::Score;
use crate::game::star::components::Star;
use crate::game::star::STAR_SIZE;
use crate::utils;

pub const PLAYER_SIZE: f32 = 64.0; // this is the size of the player sprite
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_START_HEALTH: u32 = 3;
pub const COLLISION_REBOUND_STRENGTH: f32 = 50.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
        Health {
            current: PLAYER_START_HEALTH,
        },
    ));
}

pub fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }

        direction = direction.normalize_or_zero();

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window: &Window = window_query.get_single().unwrap();
        let [x_min, x_max, y_min, y_max] = utils::get_confinement(window, PLAYER_SIZE);

        if player_transform.translation.x < x_min {
            player_transform.translation.x = x_min;
        } else if player_transform.translation.x > x_max {
            player_transform.translation.x = x_max;
        }
        if player_transform.translation.y < y_min {
            player_transform.translation.y = y_min;
        } else if player_transform.translation.y > y_max {
            player_transform.translation.y = y_max;
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&mut Transform, (With<Player>, Without<Star>)>,
    star_query: Query<(Entity, &mut Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let collision_distance = (PLAYER_SIZE + STAR_SIZE) / 2.0;
        for (star_entity, star_transform) in &star_query {
            let relative_vector_in_plane = Vec3 {
                x: player_transform.translation.x - star_transform.translation.x,
                y: player_transform.translation.y - star_transform.translation.y,
                z: 0.0,
            };

            if relative_vector_in_plane.length() > collision_distance {
                continue;
            }

            commands.spawn(AudioBundle {
                source: asset_server.load("audio/laserLarge_000.ogg"),
                settings: PlaybackSettings::DESPAWN,
            });

            commands.entity(star_entity).despawn();
            score.value += 1;
        }
    }
}

pub fn player_hit_enemy(
    mut commands: Commands,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    mut player_query: Query<(&mut Transform, &mut Health), (With<Player>, Without<Enemy>)>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut player_transform, mut player_health)) = player_query.get_single_mut() {
        let collision_distance = (PLAYER_SIZE + ENEMY_SIZE) / 2.0;
        for mut enemy_transform in &mut enemy_query {
            let mut relative_vector_in_plane = Vec3 {
                x: player_transform.translation.x - enemy_transform.translation.x,
                y: player_transform.translation.y - enemy_transform.translation.y,
                z: 0.0,
            };

            if relative_vector_in_plane.length() > collision_distance {
                continue;
            }

            commands.spawn(AudioBundle {
                source: asset_server.load("audio/explosionCrunch_000.ogg"),
                settings: PlaybackSettings::DESPAWN,
            });

            relative_vector_in_plane = relative_vector_in_plane.normalize_or_zero();
            enemy_transform.translation -= COLLISION_REBOUND_STRENGTH * relative_vector_in_plane;
            player_transform.translation += COLLISION_REBOUND_STRENGTH * relative_vector_in_plane;

            player_health.current -= 1;
            println!("You lost a health point ({} left)!", player_health.current)
        }
    }
}

pub fn check_player_health(
    mut game_over_event_writer: EventWriter<GameOver>,
    player_query: Query<&Health, With<Player>>,
    score: Res<Score>,
) {
    if let Ok(player_health) = player_query.get_single() {
        if player_health.current > 0 {
            return;
        }
        game_over_event_writer.send(GameOver { score: score.value });
    }
}

pub fn handle_game_over(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOver>,
    player_query: Query<Entity, With<Player>>,
) {
    for event in &mut game_over_event_reader {
        if let Ok(player_entity) = player_query.get_single() {
            commands.entity(player_entity).despawn();
            println!("You died! Your final score is: {}", event.score);
        }
    }
}
