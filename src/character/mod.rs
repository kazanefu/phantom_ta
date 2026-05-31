use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum CharacterKind {
    Player,
    Enemy,
    Other,
}

#[derive(Component, Clone, Copy)]
pub struct Character {
    pub kind: CharacterKind,
    pub hp: f32,
}
