use bevy::prelude::*;

use crate::{
    events::GameOver,
    game::ui::game_over_menu::{components::*, styles::*},
};

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_over_event_reader: EventReader<GameOver>,
) {
    let final_score: u32 = get_final_score(game_over_event_reader);
    let _game_over_menu_entity: Entity =
        build_game_over_menu(&mut commands, &asset_server, final_score);
}

pub fn despawn_game_over_menu(mut commands: Commands, query: Query<Entity, With<GameOverMenu>>) {
    if let Ok(game_over_menu_entity) = query.get_single() {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}

fn build_game_over_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    final_score: u32,
) -> Entity {
    commands
        .spawn((GAME_OVER_MENU_NODE, GameOverMenu))
        .with_children(|parent| {
            parent.spawn(INFO_ITEM_NODE).with_children(|parent| {
                parent.spawn((
                    ImageNode {
                        image: asset_server.load("sprites/star.png"),
                        ..default()
                    },
                    Node {
                        top: Val::Px(10.0),
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        ..default()
                    },
                ));
                parent.spawn((
                    Text::new(format! {"Final score: {:?}", final_score}),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 54.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    FinalScoreInfo,
                ));
            });
        })
        .id()
}

fn get_final_score(mut game_over_event_reader: EventReader<GameOver>) -> u32 {
    if let Some(event) = game_over_event_reader.read().next() {
        return event.score;
    }

    0
}
