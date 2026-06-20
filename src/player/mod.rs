use bevy::prelude::*;
mod control_systems;
mod input_systems;
mod status;

use crate::{
    character::{Character, CharacterKind},
    config::ControlConfig,
    ground_state::GroundState,
    player::{
        control_systems::PlayerControlPlugin, input_systems::PlayerInputPlugin,
        status::PlayerStatus,
    },
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct DashCoolTime {
    pub stock: u8,
    pub cool_time: f32,
    pub dash_time: f32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    character: Character,
    ground_state: GroundState,
    status: PlayerStatus,
    dash_cool_time: DashCoolTime,
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
            dash_cool_time: DashCoolTime {
                stock: 2,
                cool_time: 0.0,
                dash_time: 0.0,
            },
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerInputPlugin)
            .add_plugins(PlayerControlPlugin);
    }
}
