use bevy::prelude::*;
mod action_cooldown;
mod attack_hit;

pub use action_cooldown::*;
pub use attack_hit::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ActionCooldownPlugin)
            .add_plugins(AttackHitPlugin);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
