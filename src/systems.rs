use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::*;
use crate::events::*;
use crate::player::components::Player;
use crate::score::resources::Score;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
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
