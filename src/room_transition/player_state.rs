use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    GameState,
    player::{DownState, JumpingTimer, Player, PlayerRestoreBundle, PlayerSpawnBundle},
    room_transition::CurrentRoom,
    rooms::Map,
};

pub struct PlayerRoomStatePlugin;

impl Plugin for PlayerRoomStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RoomTransition), hide_player)
            .add_systems(OnEnter(GameState::Playing), place_player_at_gate);
    }
}

fn hide_player(mut commands: Commands, player_que: Query<Entity, With<Player>>) {
    for player in &player_que {
        commands.entity(player).remove::<Velocity>();
        commands.entity(player).remove::<RigidBody>();
        commands.entity(player).remove::<LockedAxes>();
        commands.entity(player).remove::<GravityScale>();
        commands.entity(player).remove::<Friction>();
        commands.entity(player).remove::<Ccd>();
        commands.entity(player).remove::<Collider>();
        commands.entity(player).remove::<CollisionGroups>();
        commands.entity(player).remove::<DownState>();
        commands.entity(player).remove::<JumpingTimer>();
        commands.entity(player).insert(Visibility::Hidden);
    }
}

fn place_player_at_gate(
    mut commands: Commands,
    player_que: Query<Entity, With<Player>>,
    map: Res<Map>,
    current_room: Res<CurrentRoom>,
) {
    let spawn_position = gate_position(&map, current_room.id).unwrap_or(Vec2::ZERO);

    if let Ok(player) = player_que.single() {
        commands.entity(player).insert(PlayerRestoreBundle::new(spawn_position));
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
