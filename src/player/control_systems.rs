use super::*;
use crate::{
    game_system_set::GameSysSet,
    ground_state::{self, GroundState},
    player::input_systems::{DashMsg, JumpMsg, MoveXDirectionMsg, RunMsg},
};
use bevy::{math::VectorSpace, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_x_velocity).in_set(GameSysSet::Logic));
    }
}

fn update_x_velocity(
    mut que: Query<(&mut Velocity, &PlayerStatus), With<Player>>,
    mut walk_msg: MessageReader<MoveXDirectionMsg>,
    mut dash_msg: MessageReader<DashMsg>,
    mut run_msg: MessageReader<RunMsg>,
) {
    for (mut velocity, status) in &mut que {
        let mut next_vel_x = 0.0;
        for left_right in walk_msg.read() {
            next_vel_x += left_right.to_sign() * status.walk_speed.value();
            if !dash_msg.is_empty() {
                next_vel_x *= status.dash_speed.value();
                dash_msg.read();
            }
            if !run_msg.is_empty() {
                next_vel_x *= status.run_speed.value();
                run_msg.read();
            }
        }
        velocity.linear.x = next_vel_x;
    }
}

fn jump_action(
    mut que: Query<(&mut Velocity, &PlayerStatus, &GroundState), With<Player>>,
    mut msg: MessageReader<JumpMsg>,
) {
    for (mut velocity, status, ground_state) in &mut que {
        if !ground_state.is_grounded() {
            continue;
        }
        for _ in msg.read() {
            velocity.linear.y = status.jump_init_speed.value();
        }
    }
}
