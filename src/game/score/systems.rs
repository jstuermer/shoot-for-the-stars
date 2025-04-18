use bevy::prelude::*;

use super::resources::*;
use crate::events::GameOver;

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in &mut game_over_event_reader.read() {
        high_scores
            .scores
            .push(("Player name:".to_string(), event.score));
    }
}
