use bevy::prelude::*;

use crate::rooms::RoomGateId;

const TRANSITION_DURATION: f32 = 1.0;

pub struct RoomTransitionPlugin;

impl Plugin for RoomTransitionPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct RoomTransition {
    pub timer: Timer,
    pub next_gate_id: RoomGateId,
}
