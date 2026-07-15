use bevy::prelude::*;

use crate::{GameState, rooms::RoomEntity};

pub struct RoomCleanupPlugin;

impl Plugin for RoomCleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RoomTransition), despawn_room_entities);
    }
}

fn despawn_room_entities(mut commands: Commands, room_entities: Query<Entity, With<RoomEntity>>) {
    for entity in &room_entities {
        commands.entity(entity).despawn();
    }
}
