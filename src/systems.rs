use bevy::{prelude::*, window::PrimaryWindow};

use crate::{game::SimulationState, AppState};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Camera2d,
    ));
}

pub fn transition_to_game_state(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        next_app_state.set(AppState::Game);
        next_simulation_state.set(SimulationState::Running);
    }
}

pub fn quit_game(
    mut app_exit_event_writer: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        app_exit_event_writer.send(AppExit::Success);
    }
}
