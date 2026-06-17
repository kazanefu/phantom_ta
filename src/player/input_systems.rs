use bevy::prelude::*;

use crate::{config::PlayerConfig, game_system_set::GameSysSet};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MoveXDirectionMsg>()
            .add_message::<JumpMsg>()
            .add_message::<DashMsg>()
            .add_message::<AttackMsg>()
            .add_message::<DownMsg>()
            .add_systems(
                Update,
                (
                    move_x_input,
                    move_y_input,
                    (dash_input, dash_send).chain(),
                    attack_input,
                )
                    .in_set(GameSysSet::Input),
            );
    }
}

#[derive(Message)]
pub enum MoveXDirectionMsg {
    Left,
    Right,
}

#[derive(Resource)]
pub struct MoveXInput {
    direction: f32,
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

#[derive(Resource)]
pub struct DashPressedTime(pub f32);

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
    mut pressed_time: ResMut<DashPressedTime>,
    mut run_msg: MessageWriter<RunMsg>,
    time: Res<Time>,
) {
    if config.input.dash.just_pressed(&keyboard, &mouse) {
        pressed_time.0 = 0.0;
    }
    if config.input.dash.pressed(&keyboard, &mouse) {
        pressed_time.0 += time.delta_secs();
        run_msg.write(RunMsg);
    }
    if config.input.dash.just_released(&keyboard, &mouse) {
        pressed_time.0 = -1.0;
    }
}

fn dash_send(
    pressed_time: ResMut<DashPressedTime>,
    config: Res<PlayerConfig>,
    mut dash_msg: MessageWriter<DashMsg>,
) {
    let range = 0.0..config.control.dash_time;
    if range.contains(&pressed_time.0) {
        dash_msg.write(DashMsg);
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
