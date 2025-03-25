use bevy::prelude::*;

use crate::{
    game::ui::pause_menu::{
        components::{ContinueButton, MainMenuButton, PauseMenu, RestartButton},
        styles::{NORMAL_BUTTON_NODE, PAUSE_MENU_NODE, TITLE_NODE},
    },
    Srgba,
};

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _game_over_menu_entity: Entity = build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    if let Ok(pause_menu_entity) = query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        // Blur gameplay
        .spawn((
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),

                ..default()
            },
            BackgroundColor(
                Srgba {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.8,
                }
                .into(),
            ),
            PauseMenu,
        ))
        .with_children(|parent| {
            parent.spawn(PAUSE_MENU_NODE).with_children(|parent| {
                // Pause menu title
                parent.spawn(TITLE_NODE).with_child((
                    Text::new("Game Paused!"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 64.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                // Continue button
                parent
                    .spawn((
                        NORMAL_BUTTON_NODE,
                        Button,
                        ContinueButton,
                        BorderRadius::all(Val::Px(10.0)),
                    ))
                    .with_child((
                        Text::new("Continue (C)"),
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
                // Restart button
                parent
                    .spawn((
                        NORMAL_BUTTON_NODE,
                        Button,
                        RestartButton,
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
                        MainMenuButton,
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
            });
        })
        // TODO: Interactions with buttons
        .id()
}
