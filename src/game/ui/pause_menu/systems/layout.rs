use bevy::prelude::*;

use crate::{game::ui::pause_menu::components::PauseMenu, Srgba};

pub fn spawn_pause_menu(mut commands: Commands) {
    let _game_over_menu_entity: Entity = build_pause_menu(&mut commands);
}

pub fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    if let Ok(pause_menu_entity) = query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

fn build_pause_menu(commands: &mut Commands) -> Entity {
    commands
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
                    alpha: 0.3,
                }
                .into(),
            ),
            PauseMenu,
        ))
        .id()
}
