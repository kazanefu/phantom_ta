use bevy::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum GameState {
    #[default]
    Loading,
    Start,
    CourseSelection,
    Playing,
    Result,
    Eixt,
}
