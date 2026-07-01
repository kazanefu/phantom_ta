use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    GameState,
    room_transition::CurrentRoom,
    rooms::{Map, items::RoomItem},
};

pub struct RoomSpawnPlugin;

impl Plugin for RoomSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnItemMsg>()
            .add_systems(OnEnter(GameState::Playing), (spawn_frame, spawn_items));
    }
}

#[derive(Message)]
pub struct SpawnItemMsg(pub RoomItem);

fn spawn_items(
    currect_room_id: Res<CurrentRoom>,
    map: Res<Map>,
    mut msg: MessageWriter<SpawnItemMsg>,
) {
    let Some(room) = map
        .rooms
        .iter()
        .find(|r| r.id == currect_room_id.id.room_id)
    else {
        return;
    };
    for item in &room.items {
        msg.write(SpawnItemMsg(*item));
    }
}

fn spawn_frame(mut commands: Commands, map: Res<Map>, current_room: Res<CurrentRoom>) {
    let Some(room) = map.rooms.iter().find(|r| r.id == current_room.id.room_id) else {
        println!(
            "fail to spawn frame for room id: {}",
            current_room.id.room_id
        );
        return;
    };
    const ONE_BOX: f32 = 50.0;
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(room.range.x, ONE_BOX)),
            ..default()
        },
        Collider::cuboid(room.range.x / 2.0, ONE_BOX / 2.0),
        RigidBody::Fixed,
        Transform::from_xyz(0.0, room.range.y / 2.0, 0.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(room.range.x, ONE_BOX)),
            ..default()
        },
        Collider::cuboid(room.range.x / 2.0, ONE_BOX / 2.0),
        RigidBody::Fixed,
        Transform::from_xyz(0.0, -room.range.y / 2.0, 0.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(ONE_BOX, room.range.y)),
            ..default()
        },
        Collider::cuboid(ONE_BOX / 2.0, room.range.y / 2.0),
        RigidBody::Fixed,
        Transform::from_xyz(room.range.x / 2.0, 0.0, 0.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(ONE_BOX, room.range.y)),
            ..default()
        },
        Collider::cuboid(ONE_BOX / 2.0, room.range.y / 2.0),
        RigidBody::Fixed,
        Transform::from_xyz(-room.range.x / 2.0, 0.0, 0.0),
    ));
}
