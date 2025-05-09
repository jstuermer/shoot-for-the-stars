use std::fmt::Write;

use bevy::prelude::*;

use crate::game::{
    components::Health,
    enemy::components::Enemy,
    player::components::Player,
    score::resources::Score,
    ui::hud::components::{ControlsHUD, EnemyNumberInfo, GameInfoHUD, HealthInfo, ScoreInfo},
};

pub fn update_score_info(
    mut score_info_query: Query<&mut Text, With<ScoreInfo>>,
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut text) = score_info_query.get_single_mut() {
            text.clear();
            let _ = text.write_str(&format!("{:?}", score.value));
        }
    }
}

pub fn update_health_info(
    mut health_info_query: Query<&mut Text, With<HealthInfo>>,
    player_health_query: Query<&Health, (Changed<Health>, With<Player>)>,
) {
    if let Ok(player_health) = player_health_query.get_single() {
        if let Ok(mut text) = health_info_query.get_single_mut() {
            text.clear();
            let _ = text.write_str(&format!("{:?}", player_health.current));
        }
    }
}

pub fn update_enemy_number_info(
    mut enemy_number_info_query: Query<&mut Text, With<EnemyNumberInfo>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if let Ok(mut text) = enemy_number_info_query.get_single_mut() {
        let number_of_enemies = enemy_query.iter().count();
        text.clear();
        let _ = text.write_str(&format!("{:?}", number_of_enemies));
    }
}

pub fn despawn_game_hud(
    mut commands: Commands,
    game_info_query: Query<Entity, With<GameInfoHUD>>,
    controls_query: Query<Entity, With<ControlsHUD>>,
) {
    if let Ok(game_info_entity) = game_info_query.get_single() {
        commands.entity(game_info_entity).despawn_recursive();
    }
    if let Ok(controls_entity) = controls_query.get_single() {
        commands.entity(controls_entity).despawn_recursive();
    }
}
