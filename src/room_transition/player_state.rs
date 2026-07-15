use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionGroups, GravityScale, Group, Velocity};

use crate::{
    GameState,
    player::{GateSpawnImmunity, Player, PlayerSpawnBundle},
    room_transition::{CurrentRoom, RoomTransition, RoomTransitionSet, TransitionPhase},
    rooms::Map,
};

pub struct PlayerRoomStatePlugin;

impl Plugin for PlayerRoomStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            hide_player
                .in_set(RoomTransitionSet::PhaseWork)
                .run_if(in_state(GameState::RoomTransition)),
        )
        .add_systems(OnEnter(GameState::Playing), place_player_at_gate);
    }
}

fn hide_player(
    transition: Res<RoomTransition>,
    mut player_que: Query<
        (
            &mut Visibility,
            &mut CollisionGroups,
            &mut Velocity,
            &mut GravityScale,
        ),
        With<Player>,
    >,
) {
    if transition.phase != TransitionPhase::Cleanup {
        return;
    }

    for (mut visibility, mut collision_groups, mut velocity, mut gravity_scale) in &mut player_que {
        *visibility = Visibility::Hidden;
        collision_groups.filters = Group::NONE;
        velocity.linear = Vec2::ZERO;
        gravity_scale.0 = 0.0;
    }
}

fn place_player_at_gate(
    mut commands: Commands,
    mut player_que: Query<
        (
            Entity,
            &mut Transform,
            &mut Visibility,
            &mut CollisionGroups,
            &mut Velocity,
            &mut GravityScale,
        ),
        With<Player>,
    >,
    map: Res<Map>,
    current_room: Res<CurrentRoom>,
) {
    let spawn_position = gate_position(&map, current_room.id).unwrap_or(Vec2::ZERO);

    if let Ok((
        player,
        mut transform,
        mut visibility,
        mut collision_groups,
        mut velocity,
        mut gravity_scale,
    )) = player_que.single_mut()
    {
        transform.translation = spawn_position.extend(0.0);
        *visibility = Visibility::Visible;
        collision_groups.filters = Group::all();
        velocity.linear = Vec2::ZERO;
        gravity_scale.0 = 40.0;
        commands.entity(player).insert(GateSpawnImmunity);
    } else {
        commands.spawn(PlayerSpawnBundle::new(spawn_position));
    }
}

fn gate_position(map: &Map, gate_id: crate::rooms::RoomGateId) -> Option<Vec2> {
    let room = map.rooms.iter().find(|room| room.id == gate_id.room_id)?;
    room.gates
        .iter()
        .find(|gate| gate.id == gate_id)
        .or_else(|| room.gates.first())
        .map(|gate| gate.position)
}
