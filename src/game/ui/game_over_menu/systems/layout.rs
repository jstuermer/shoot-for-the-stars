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
            // Game over menu title
            parent.spawn(TITLE_NODE).with_child((
                Text::new("Game Over!"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            // Final score info
            parent.spawn(INFO_ITEM_NODE).with_children(|parent| {
                parent.spawn((
                    ImageNode {
                        image: asset_server.load("sprites/star.png"),
                        ..default()
                    },
                    Node {
                        top: Val::Px(10.0),
                        width: Val::Px(35.0),
                        height: Val::Px(35.0),
                        ..default()
                    },
                ));
                parent.spawn((
                    Text::new(format! {"Final score: {:?}", final_score}),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 44.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    FinalScoreInfo,
                ));
            });
            // Restart button
            parent
                .spawn((
                    NORMAL_BUTTON_NODE,
                    Button,
                    GameOverRestartButton,
                    BorderRadius::all(Val::Px(10.0)),
                ))
                .with_child((
                    Text::new("Restart (R)"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout {
                        justify: JustifyText::Center,
                        ..default()
                    },
                ));
            // Main menu button
            parent
                .spawn((
                    NORMAL_BUTTON_NODE,
                    Button,
                    GameOverMainMenuButton,
                    BorderRadius::all(Val::Px(10.0)),
                ))
                .with_child((
                    Text::new("Main Menu (M)"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout {
                        justify: JustifyText::Center,
                        ..default()
                    },
                ));
        })
        .id()
}

fn get_final_score(mut game_over_event_reader: EventReader<GameOver>) -> u32 {
    if let Some(event) = game_over_event_reader.read().next() {
        return event.score;
    }

    0
}
