use bevy::prelude::*;

use crate::{GameState, rooms::RoomGateId};

mod player_state;
mod room_cleanup;
mod camera_state;
mod transition_screen;

const TRANSITION_DURATION: f32 = 1.0;

pub struct RoomTransitionPlugin;

impl Plugin for RoomTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoomTransition>()
            .init_resource::<CurrentRoom>()
            .add_systems(OnEnter(GameState::RoomTransition), reset_timer_system)
            .add_plugins((
                player_state::PlayerRoomStatePlugin,
                room_cleanup::RoomCleanupPlugin,
                camera_state::CameraRoomStatePlugin,
                transition_screen::TransitionScreenPlugin,
            ))
            .add_systems(
                Update,
                poll_transition_system.run_if(in_state(GameState::RoomTransition)),
            );
    }
}

#[derive(Resource)]
pub struct RoomTransition {
    pub timer: Timer,
    next_gate_id: Option<RoomGateId>,
}

#[derive(Resource, Default)]
pub struct CurrentRoom {
    pub id: RoomGateId,
}
impl Default for RoomTransition {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(TRANSITION_DURATION, TimerMode::Once),
            next_gate_id: None,
        }
    }
}
impl RoomTransition {
    pub fn set_next_gate_id(&mut self, next_gate_id: RoomGateId) {
        self.next_gate_id = Some(next_gate_id);
    }

    fn reset(&mut self) {
        self.timer.reset();
    }
}

fn reset_timer_system(mut transition: ResMut<RoomTransition>) {
    transition.reset();
}

fn poll_transition_system(
    time: Res<Time>,
    mut transition: ResMut<RoomTransition>,
    mut current_room: ResMut<CurrentRoom>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    transition.timer.tick(time.delta());
    if transition.timer.is_finished() {
        if let Some(next_gate_id) = transition.next_gate_id {
            current_room.id = next_gate_id;
            transition.next_gate_id = None;
        }
        next_state.set(GameState::Playing);
    }
}
