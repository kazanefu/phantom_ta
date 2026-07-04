use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    GameState,
    ground_state::Ground,
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
    let rx = room.range.x;
    let ry = room.range.y;

    // def (width, height, x, y)
    let walls = [
        (rx, ONE_BOX, 0.0, ry / 2.0),  // up wall
        (rx, ONE_BOX, 0.0, -ry / 2.0), // down wall
        (ONE_BOX, ry, rx / 2.0, 0.0),  // right wall
        (ONE_BOX, ry, -rx / 2.0, 0.0), // left wall
    ];

    for (size_x, size_y, pos_x, pos_y) in walls {
        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(size_x, size_y)),
                ..default()
            },
            Collider::cuboid(size_x / 2.0, size_y / 2.0),
            RigidBody::Fixed,
            Transform::from_xyz(pos_x, pos_y, 0.0),
            Ground,
        ));
    }
}
