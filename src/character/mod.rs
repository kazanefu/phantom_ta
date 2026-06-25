use bevy::prelude::*;
mod action_cooldown;

pub use action_cooldown::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ActionCooldownPlugin);
    }
}

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
