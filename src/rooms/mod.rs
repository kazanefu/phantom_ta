use bevy::prelude::*;

pub use items::RoomItem;
use serde::{Deserialize, Serialize};

use crate::{
    GameState,
    file::{Ron, SaveLoad},
    loading::LoadTaskState,
    loading::TaskState,
};

mod items;
mod spawn;
pub use items::ItemKind;
pub use spawn::SpawnItemMsg;

pub struct RoomsPlugin;
impl Plugin for RoomsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spawn::RoomSpawnPlugin)
            .add_systems(
                Update,
                load_map.run_if(in_state(GameState::Loading).and(
                    |tasklist: Res<LoadTaskState>| {
                        !tasklist.is_task_done(crate::loading::LoadTaskKind::Map)
                    },
                )),
            )
            .add_plugins(items::RoomItemsPlugin);
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomGateId {
    pub room_id: usize,
    pub gate_id: usize,
}
#[derive(Serialize, Deserialize)]
pub struct RoomGate {
    pub id: RoomGateId,
    pub position: Vec2,
    // Option: because start gate has no next gate. but other gates do.
    pub next_gate: Option<RoomGateId>,
}

#[derive(Serialize, Deserialize)]
pub struct Room {
    pub id: usize,
    pub gates: Vec<RoomGate>,
    pub range: Vec2, // aabb((0,0) -> (x,y))
    pub items: Vec<RoomItem>,
    pub enemies: Vec<crate::enemy::Enemy>,
}

#[derive(Serialize, Deserialize, Default, Resource)]
pub struct Map {
    pub rooms: Vec<Room>,
}

impl SaveLoad for Map {
    const PATH: &'static str = "assets/map.ron";
    type Format = Ron;
    const USE_APPLICATION_DATA_DIR: bool = false;
}

fn load_map(mut commands: Commands, mut tasklist: ResMut<LoadTaskState>) {
    let map = Map::load_default_path().unwrap_or_default();
    commands.insert_resource(map);
    tasklist.set_task_done(crate::loading::LoadTaskKind::Map);
}
