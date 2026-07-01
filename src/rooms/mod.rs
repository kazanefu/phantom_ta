use bevy::prelude::*;

use items::Item;
use serde::{Deserialize, Serialize};

use crate::file::{Ron, SaveLoad};

mod items;

#[derive(Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomGateId {
    pub room_id: usize,
    pub gate_id: usize,
}
#[derive(Serialize, Deserialize)]
pub struct RoomGate {
    pub id: RoomGateId,
    pub position: Vec2,
    pub next_gate: RoomGateId,
}

#[derive(Serialize, Deserialize)]
pub struct Room {
    pub id: usize,
    pub gates: Vec<RoomGate>,
    pub range: Vec2, // aabb((0,0) -> (x,y))
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Map {
    pub rooms: Vec<Room>,
}

impl SaveLoad for Map {
    const PATH: &'static str = "assets/map.ron";
    type Format = Ron;
    const USE_APPLICATION_DATA_DIR: bool = false;
}
