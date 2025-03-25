use bevy::prelude::*;

use crate::{
    game::{
        ui::pause_menu::{
            components::{ContinueButton, MainMenuButton, RestartButton},
            styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
        },
        SimulationState,
    },
    AppState,
};

type ContinueButtonInteraction = (Changed<Interaction>, With<ContinueButton>);

pub fn interact_with_continue_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), ContinueButtonInteraction>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_simulation_state.set(SimulationState::Running);
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

type RestartButtonInteraction = (Changed<Interaction>, With<RestartButton>);

pub fn interact_with_restart_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), RestartButtonInteraction>,
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

type MainMenuButtonInteraction = (Changed<Interaction>, With<MainMenuButton>);

pub fn interact_with_main_menu_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), MainMenuButtonInteraction>,
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

pub fn continue_game(
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        next_simulation_state.set(SimulationState::Running);
    }
}

pub fn reset_game(
    mut next_app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        next_app_state.set(AppState::Restarting);
        println!("Game is resetting!");
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
        println!("Game is restarting!");
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
