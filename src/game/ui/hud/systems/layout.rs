use bevy::prelude::*;

use crate::game::{
    enemy::NUMBER_OF_ENEMIES,
    player::PLAYER_START_HEALTH,
    score::resources::Score,
    ui::hud::{components::*, styles::*},
};

pub fn spawn_game_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _hud_entity: Entity = build_hud(&mut commands, &asset_server);
}

fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((HUD_NODE, GameHUD))
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
