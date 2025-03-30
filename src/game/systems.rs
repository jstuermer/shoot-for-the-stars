use bevy::prelude::*;

use super::{components::Velocity, SimulationState};

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

pub fn step_physics(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time<Fixed>>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.current * time.delta_secs();
    }
}
