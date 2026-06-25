use std::collections::HashSet;

use bevy::prelude::*;
use bevy_rapier2d::{dynamics::Velocity, prelude::*};
mod collision;
mod control_systems;
mod input_systems;
mod save_data;
mod skills;
mod status;

use crate::{
    PLAYER_GROUP,
    character::{Character, CharacterKind},
    config::ControlConfig,
    ground_state::GroundState,
    player::{
        collision::PlayerCollisionPlugin, control_systems::PlayerControlPlugin,
        input_systems::PlayerInputPlugin, skills::SkillPlugin, status::PlayerStatus,
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

#[derive(Component, Default)]
pub struct JumpingTimer {
    jumping_time: f32,
    hold_time: f32,
}
#[derive(Component)]
pub struct DownState(pub bool);

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    character: Character,
    ground_state: GroundState,
    status: PlayerStatus,
    dash_cool_time: DashCoolTime,
    vel: Velocity,
    rigidbody: RigidBody,
    axis_lock: LockedAxes,
    gravity: GravityScale,
    friction: Friction,
    ccd: Ccd,
    jumping_timer: JumpingTimer,
    down_state: DownState,
    collision_group: CollisionGroups,
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
            vel: Velocity::default(),
            rigidbody: RigidBody::Dynamic,
            axis_lock: LockedAxes::ROTATION_LOCKED,
            gravity: GravityScale(40.0),
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ccd: Ccd::enabled(),
            jumping_timer: JumpingTimer::default(),
            down_state: DownState(false),
            collision_group: CollisionGroups::new(PLAYER_GROUP, Group::all()),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<save_data::PlayerSaveData>()
            .add_plugins(PlayerInputPlugin)
            .add_plugins(PlayerCollisionPlugin)
            .add_plugins(PlayerControlPlugin)
            .add_plugins(SkillPlugin);
    }
}
