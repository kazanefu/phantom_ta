use bevy::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum GameState {
    #[default]
    Loading,
    Start,
    SaveDataSelect,
    Playing,
    RoomTransition,
    Result,
    Exit,
}
