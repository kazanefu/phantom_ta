use bevy::prelude::*;

use crate::{GameState, time::TimeState};

pub struct RoomTransitionTimeControlPlugin;

impl Plugin for RoomTransitionTimeControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RoomTransition), pause_game_time)
            .add_systems(OnEnter(GameState::Playing), resume_game_time);
    }
}

fn pause_game_time(mut next_time_state: ResMut<NextState<TimeState>>) {
    next_time_state.set(TimeState::Paused);
}

fn resume_game_time(mut next_time_state: ResMut<NextState<TimeState>>) {
    next_time_state.set(TimeState::Running);
}
