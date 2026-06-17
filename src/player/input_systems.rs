use bevy::prelude::*;

use crate::{config::PlayerConfig, game_system_set::GameSysSet};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_x_input, move_y_input, dash_input, attack_input).in_set(GameSysSet::Input),
        );
    }
}

#[derive(Message)]
pub enum MoveXDirectionMsg {
    Left,
    Right,
}

impl MoveXDirectionMsg {
    pub fn to_sign(&self) -> f32 {
        match self {
            MoveXDirectionMsg::Left => -1.0,
            MoveXDirectionMsg::Right => 1.0,
        }
    }
    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.to_sign(), 0.0)
    }
}

#[derive(Message)]
pub struct JumpMsg;

#[derive(Message)]
pub struct DownMsg;

#[derive(Message)]
pub struct DashMsg;

#[derive(Message)]
pub enum AttackMsg {
    Start,
    End,
}

fn move_x_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    config: Res<PlayerConfig>,
    mut msg: MessageWriter<MoveXDirectionMsg>,
) {
    match (
        config.input.left.pressed(&keyboard, &mouse),
        config.input.right.pressed(&keyboard, &mouse),
    ) {
        (true, false) => {
            msg.write(MoveXDirectionMsg::Left);
        }
        (false, true) => {
            msg.write(MoveXDirectionMsg::Right);
        }
        _ => {}
    }
}

fn move_y_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    config: Res<PlayerConfig>,
    mut jump_msg: MessageWriter<JumpMsg>,
    mut down_msg: MessageWriter<DownMsg>,
) {
    match (
        config.input.jump.just_pressed(&keyboard, &mouse),
        config.input.down.just_pressed(&keyboard, &mouse),
    ) {
        (true, false) => {
            jump_msg.write(JumpMsg);
        }
        (false, true) => {
            down_msg.write(DownMsg);
        }
        _ => {}
    }
}

fn dash_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    config: Res<PlayerConfig>,
    mut msg: MessageWriter<DashMsg>,
) {
    if config.input.dash.just_pressed(&keyboard, &mouse) {
        msg.write(DashMsg);
    }
}

fn attack_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    config: Res<PlayerConfig>,
    mut msg: MessageWriter<AttackMsg>,
) {
    if config.input.attack.just_pressed(&keyboard, &mouse) {
        msg.write(AttackMsg::Start);
    }
    if config.input.attack.just_released(&keyboard, &mouse) {
        msg.write(AttackMsg::End);
    }
}
