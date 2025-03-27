use bevy::prelude::*;

use crate::game::{
    enemy::NUMBER_OF_ENEMIES,
    player::PLAYER_START_HEALTH,
    score::resources::Score,
    ui::hud::{components::*, styles::*},
};

pub fn spawn_game_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _hud_entity: Entity = build_info_hud(&mut commands, &asset_server);
    let _controls_entity: Entity = build_controls_hud(&mut commands, &asset_server);
}

fn build_info_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((INFO_HUD_NODE, GameInfoHUD))
        // Info bar at the top-left of the screen
        .with_children(|parent| {
            parent
                .spawn((
                    INFO_BAR_NODE,
                    BorderRadius::all(Val::Px(10.0)),
                    BackgroundColor(INFO_BAR_COLOR.into()),
                ))
                .with_children(|parent| {
                    // Score info at the top
                    parent.spawn(INFO_ITEM_NODE).with_children(|parent| {
                        parent.spawn((
                            ImageNode {
                                image: asset_server.load("sprites/star.png"),
                                ..default()
                            },
                            Node {
                                top: Val::Px(3.0),
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                ..default()
                            },
                        ));
                        parent.spawn((
                            Text::new(format!("{:?}", Score::default().value)),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            ScoreInfo,
                        ));
                    });
                    // Player health info in the center
                    parent.spawn(INFO_ITEM_NODE).with_children(|parent| {
                        parent.spawn((
                            ImageNode {
                                image: asset_server.load("sprites/info_heart.png"),
                                ..default()
                            },
                            Node {
                                top: Val::Px(5.0),
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                ..default()
                            },
                        ));
                        parent.spawn((
                            Text::new(format!("{:?}", PLAYER_START_HEALTH)),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            HealthInfo,
                        ));
                    });
                    // Number of enemies info at the bottom
                    parent.spawn(INFO_ITEM_NODE).with_children(|parent| {
                        parent.spawn((
                            ImageNode {
                                image: asset_server.load("sprites/ball_red_large.png"),
                                ..default()
                            },
                            Node {
                                top: Val::Px(3.0),
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                ..default()
                            },
                        ));
                        parent.spawn((
                            Text::new(format!("{:?}", NUMBER_OF_ENEMIES)),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            EnemyNumberInfo,
                        ));
                    });
                })
                .with_child((INFO_BAR_NODE, BackgroundColor(INFO_BAR_COLOR.into())));
        })
        .id()
}

fn build_controls_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                ..default()
            },
            ControlsHUD,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("W: Move up"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(CONTROLS_INFO_COLORS.into()),
                    ));
                    parent.spawn((
                        Text::new("A: Move left"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(CONTROLS_INFO_COLORS.into()),
                    ));
                    parent.spawn((
                        Text::new("S: Move down"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(CONTROLS_INFO_COLORS.into()),
                    ));
                    parent.spawn((
                        Text::new("D: Move right"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(CONTROLS_INFO_COLORS.into()),
                    ));
                    parent.spawn((
                        Text::new("Space: Pause"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(CONTROLS_INFO_COLORS.into()),
                    ));
                });
        })
        .id()
}
