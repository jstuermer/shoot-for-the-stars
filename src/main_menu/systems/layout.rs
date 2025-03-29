use bevy::prelude::*;

use crate::{
    game::player::PLAYER_SPRITE,
    main_menu::{
        components::{MainMenu, PlayButton, QuitButton},
        styles::*,
    },
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _main_menu_entity: Entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((MAIN_MENU_NODE, MainMenu))
        .with_children(|parent| {
            // Title
            parent.spawn(TITLE_NODE).with_children(|parent| {
                parent.spawn(ImageNode {
                    image: asset_server.load(PLAYER_SPRITE),
                    ..default()
                });
                parent.spawn((
                    Text::new("Shoot For The Stars"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 64.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                parent.spawn(ImageNode {
                    image: asset_server.load(PLAYER_SPRITE),
                    ..default()
                });
            });
            // Play button
            parent
                .spawn((
                    NORMAL_BUTTON_NODE,
                    Button,
                    PlayButton,
                    BorderRadius::all(Val::Px(10.0)),
                ))
                .with_child((
                    Text::new("Play (P)"),
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
            // Quit button
            parent
                .spawn((
                    NORMAL_BUTTON_NODE,
                    Button,
                    QuitButton,
                    BorderRadius::all(Val::Px(10.0)),
                ))
                .with_child((
                    Text::new("Quit (Q)"),
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
