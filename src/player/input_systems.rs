use bevy::prelude::*;

use crate::{config::PlayerConfig, game_system_set::GameSysSet};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<JumpMsg>()
            .add_message::<AttackMsg>()
            .init_resource::<DownInput>()
            .add_message::<DashMsg>()
            .init_resource::<JumpHoldTime>()
            .insert_resource(MoveXInput {
                direction: 0.0,
                is_running: false,
            })
            .add_systems(
                Update,
                (
                    move_x_input,
                    (jump_hold, move_y_input).chain(),
                    dash_input,
                    attack_input,
                )
                    .in_set(GameSysSet::Input),
            );
    }
}

#[derive(Resource)]
pub struct MoveXInput {
    pub direction: f32,
    pub is_running: bool,
}

#[derive(Message)]
pub struct DashMsg;

#[derive(Message)]
pub struct JumpMsg(pub f32);

#[derive(Resource, Default)]
pub struct JumpHoldTime(pub f32);

#[derive(Resource, Default)]
pub struct DownInput(pub bool);

#[derive(Message)]
pub enum AttackMsg {
    Start,
    End,
}

fn move_x_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    config: Res<PlayerConfig>,
    mut input_record: ResMut<MoveXInput>,
) {
    match (
        config.input.left.pressed(&keyboard, &mouse),
        config.input.right.pressed(&keyboard, &mouse),
    ) {
        (true, false) => {
            input_record.direction = -1.0;
        }
        (false, true) => {
            input_record.direction = 1.0;
        }
        _ => {
            input_record.direction = 0.0;
        }
    }
}

fn jump_hold(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    config: Res<PlayerConfig>,
    mut hold_time: ResMut<JumpHoldTime>,
    time: Res<Time>,
) {
    if config.input.jump.pressed(&keyboard, &mouse) {
        hold_time.0 += time.delta_secs();
    }
}

fn move_y_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    config: Res<PlayerConfig>,
    mut jump_msg: MessageWriter<JumpMsg>,
    mut down: ResMut<DownInput>,
    mut hold_time: ResMut<JumpHoldTime>,
) {
    down.0 = false;
    match (
        config.input.jump.just_released(&keyboard, &mouse),
        config.input.down.pressed(&keyboard, &mouse),
    ) {
        (true, false) => {
            jump_msg.write(JumpMsg(hold_time.0));
            hold_time.0 = 0.0;
        }
        (false, true) => {
            down.0 = true;
        }
        _ => {}
    }
}

fn dash_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    config: Res<PlayerConfig>,
    mut input_handler: ResMut<MoveXInput>,
    mut dash_msg: MessageWriter<DashMsg>,
) {
    if config.input.dash.just_pressed(&keyboard, &mouse) {
        dash_msg.write(DashMsg);
    }
    if config.input.dash.pressed(&keyboard, &mouse) {
        input_handler.is_running = true;
    } else {
        input_handler.is_running = false;
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
