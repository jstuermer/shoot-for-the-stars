use bevy::prelude::*;

use crate::{
    game::{
        ui::game_over_menu::{
            components::{GameOverMainMenuButton, GameOverRestartButton},
            styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
        },
        SimulationState,
    },
    AppState,
};

type GameOverRestartButtonInteraction = (Changed<Interaction>, With<GameOverRestartButton>);

pub fn interact_with_restart_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), GameOverRestartButtonInteraction>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

type GameOverMainMenuButtonInteraction = (Changed<Interaction>, With<GameOverMainMenuButton>);

pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        GameOverMainMenuButtonInteraction,
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn reset_game(
    mut next_app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        next_app_state.set(AppState::Restarting);
    }
}

pub fn restart_game(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        next_app_state.set(AppState::Game);
        next_simulation_state.set(SimulationState::Running);
    }
}

pub fn transition_to_main_menu_state(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        next_app_state.set(AppState::MainMenu);
        next_simulation_state.set(SimulationState::Paused);
    }
}
