use bevy::prelude::*;

use crate::game::{
    components::Health,
    player::components::Player,
    score::resources::Score,
    ui::hud::components::{GameHUD, HealthInfo, ScoreInfo},
};

pub fn update_score_info(
    mut score_info_query: Query<&mut Text, With<ScoreInfo>>,
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut text) = score_info_query.get_single_mut() {
            text.sections[0].value = format!("{:?}", score.value);
        }
    }
}

pub fn update_health_info(
    mut health_info_query: Query<&mut Text, With<HealthInfo>>,
    player_health_query: Query<&Health, (Changed<Health>, With<Player>)>,
) {
    if let Ok(player_health) = player_health_query.get_single() {
        if let Ok(mut text) = health_info_query.get_single_mut() {
            text.sections[0].value = format!("{:?}", player_health.current);
        }
    }
}

pub fn despawn_game_hud(mut commands: Commands, query: Query<Entity, With<GameHUD>>) {
    if let Ok(game_hud_entity) = query.get_single() {
        commands.entity(game_hud_entity).despawn_recursive();
    }
}
