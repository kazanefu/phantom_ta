use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collision_groups::GATE_GROUP,
    rooms::{Map, RoomEntity, RoomGate, RoomGateId},
};

pub struct RoomGatePlugin;

impl Plugin for RoomGatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::GameState::Playing), spawn_room_gates);
    }
}

#[derive(Component, Clone, Copy)]
pub struct Gate {
    pub id: RoomGateId,
    pub next_gate: Option<RoomGateId>,
}

fn spawn_room_gates(
    mut commands: Commands,
    map: Res<Map>,
    current_room: Res<crate::room_transition::CurrentRoom>,
) {
    let Some(room) = map.rooms.iter().find(|room| room.id == current_room.id.room_id) else {
        return;
    };

    for gate in &room.gates {
        spawn_gate(&mut commands, gate);
    }
}

fn spawn_gate(commands: &mut Commands, gate: &RoomGate) {
    let size = Vec2::new(60.0, 120.0);
    commands.spawn((
        Gate {
            id: gate.id,
            next_gate: gate.next_gate,
        },
        RoomEntity,
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        CollisionGroups::new(GATE_GROUP, crate::PLAYER_GROUP),
        Collider::cuboid(size.x / 2.0, size.y / 2.0),
        Sprite {
            color: Color::srgb(0.9, 0.2, 0.9),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(gate.position.x, gate.position.y, 1.0),
    ));
}
