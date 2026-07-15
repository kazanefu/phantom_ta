use bevy::prelude::*;

use crate::{
    GameState,
    room_transition::{RoomTransition, RoomTransitionSet, TransitionPhase},
    rooms::RoomEntity,
};

pub struct RoomCleanupPlugin;

impl Plugin for RoomCleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            despawn_room_entities
                .in_set(RoomTransitionSet::PhaseWork)
                .run_if(in_state(GameState::RoomTransition)),
        );
    }
}

fn despawn_room_entities(
    mut commands: Commands,
    transition: Res<RoomTransition>,
    room_entities: Query<Entity, With<RoomEntity>>,
) {
    if transition.phase != TransitionPhase::Cleanup {
        return;
    }
    for entity in &room_entities {
        commands.entity(entity).despawn();
    }
}
