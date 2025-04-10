use bevy::{prelude::*, window::PrimaryWindow};

use crate::utils;

use super::{
    components::{Size, Velocity},
    SimulationState,
};

pub fn toggle_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    simulation_state: Res<State<SimulationState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation is paused.");
        } else if *simulation_state.get() == SimulationState::Paused {
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation is running.");
        }
    }
}

/// Update the positions of all entities that have a `Velocity` component.
pub fn apply_velocities(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time<Fixed>>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.current * time.delta_secs();
    }
}

/// Confine entities to the window size.
pub fn enforce_spatial_confinement(
    mut query: Query<(&mut Transform, &mut Velocity, &Size)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for (mut transform, mut velocity, size) in &mut query {
        let [x_min, x_max, y_min, y_max] = utils::get_confinement(window, size.width, size.height);

        if transform.translation.x < x_min {
            transform.translation.x = x_min;
            velocity.current.x = -velocity.current.x;
        } else if transform.translation.x > x_max {
            transform.translation.x = x_max;
            velocity.current.x = -velocity.current.x;
        }
        if transform.translation.y < y_min {
            transform.translation.y = y_min;
            velocity.current.y = -velocity.current.y;
        } else if transform.translation.y > y_max {
            transform.translation.y = y_max;
            velocity.current.y = -velocity.current.y;
        }
    }
}
