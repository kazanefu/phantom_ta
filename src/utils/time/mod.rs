use bevy::prelude::*;

mod time_state;

pub use time_state::TimeState;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TimeState>()
            .configure_sets(
                Update,
                bevy_rapier2d::prelude::PhysicsSet::StepSimulation
                    .run_if(in_state(TimeState::Running)),
            )
            .add_systems(OnEnter(TimeState::Paused), time_state::pause)
            .add_systems(OnEnter(TimeState::Running), time_state::unpause);
    }
}
