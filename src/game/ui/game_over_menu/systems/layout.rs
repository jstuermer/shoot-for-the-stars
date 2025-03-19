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

fn build_game_over_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    final_score: u32,
) -> Entity {
    let game_over_menu_entity: Entity = commands
        .spawn((
            NodeBundle {
                style: GAME_OVER_MENU_STYLE,
                background_color: Color::GRAY.into(),
                ..default()
            },
            GameOverMenu {},
        ))
        // Final score
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: INFO_ITEM_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    let icon = asset_server.load("sprites/star.png");
                    parent.spawn(ImageBundle {
                        image: UiImage::new(icon),
                        style: Style {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn((
                        TextBundle::from_section(
                            format!("Final score: {:?}", final_score),
                            get_text_style(54.0, &asset_server),
                        ),
                        FinalScoreInfo {},
                    ));
                });
        })
        .id();

    return game_over_menu_entity;
}

fn get_final_score(mut game_over_event_reader: EventReader<GameOver>) -> u32 {
    for event in &mut game_over_event_reader {
        return event.score;
    }

    return 0;
}
