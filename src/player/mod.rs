use bevy::prelude::*;
mod control_systems;
mod input_systems;
mod status;

use crate::{
    character::{Character, CharacterKind},
    config::ControlConfig,
    ground_state::GroundState,
    player::status::PlayerStatus,
};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    character: Character,
    ground_state: GroundState,
    status: PlayerStatus,
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
            status: PlayerStatus::from_config(&ControlConfig::default()),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {}
}
