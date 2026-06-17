use bevy::prelude::*;
mod control_systems;
mod input_systems;

use crate::{
    character::{Character, CharacterKind},
    ground_state::GroundState,
};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    character: Character,
    ground_state: GroundState,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            character: Character {
                kind: CharacterKind::Player,
                hp: 100.0,
            },
            ground_state: GroundState::default(),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}
