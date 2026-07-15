use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    game_system_set::GameSysSet,
    player::{GateSpawnImmunity, Player},
    room_transition::RoomTransition,
    rooms::Gate,
    utils::collision::get_contained_entity,
    GameState,
};

pub fn touch_gate(
    mut events: MessageReader<CollisionEvent>,
    player_que: Query<(), (With<Player>, Without<GateSpawnImmunity>)>,
    gate_que: Query<&Gate>,
    mut transition: ResMut<RoomTransition>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for &event in events.read() {
        let CollisionEvent::Started(e1, e2, _) = event else {
            continue;
        };
        if get_contained_entity(e1, e2, &player_que).is_none() {
            continue;
        }
        let Some(gate_entity) = get_contained_entity(e1, e2, &gate_que) else {
            continue;
        };
        let Ok(gate) = gate_que.get(gate_entity) else {
            continue;
        };
        let Some(next_gate) = gate.next_gate else {
            continue;
        };
        transition.set_next_gate_id(next_gate);
        next_state.set(GameState::RoomTransition);
    }
}

pub struct GateCollisionPlugin;

impl Plugin for GateCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            touch_gate
                .in_set(GameSysSet::Detection)
                .run_if(in_state(GameState::Playing)),
        );
    }
}
