use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{
    config::PlayerConfig,
    game_system_set::GameSysSet,
    player::{Player, PlayerSaveData, skills::SkillStack},
    time::TimeState,
};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<JumpMsg>()
            .add_message::<AttackMsg>()
            .init_resource::<DownInput>()
            .add_message::<DashMsg>()
            .init_resource::<JumpHoldTime>()
            .init_resource::<MousePosition>()
            .insert_resource(MoveXInput {
                direction: 0.0,
                is_running: false,
            })
            .add_systems(
                Update,
                (
                    update_mouse_position,
                    move_x_input,
                    (jump_hold, move_y_input).chain(),
                    dash_input,
                    attack_input,
                    skill_push_input,
                )
                    .in_set(GameSysSet::Input)
                    .run_if(in_state(TimeState::Running)),
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

#[derive(Message, PartialEq, Eq, Clone, Copy)]
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

#[derive(Resource, Default)]
pub struct MousePosition {
    pub position: Vec2,
    pub delta: Vec2,
    pub direction: Vec2,
}

fn update_mouse_position(
    windows: Query<&Window>,
    mut mouse_position: ResMut<MousePosition>,
    player_que: Query<&Transform, With<Player>>,
    camera_que: Query<(&Camera, &GlobalTransform)>,
    mut mouse_motion: MessageReader<MouseMotion>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok(player_transform) = player_que.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_que.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok(mouse_world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    else {
        return;
    };
    mouse_position.position = mouse_world_position;
    mouse_position.direction =
        (mouse_world_position - player_transform.translation.truncate()).normalize_or_zero();
    mouse_position.delta = Vec2::ZERO;
    for motion in mouse_motion.read() {
        mouse_position.delta += motion.delta;
    }
}

fn skill_push_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    config: Res<PlayerConfig>,
    mut skill_stack: ResMut<SkillStack>,
    player_data: Res<PlayerSaveData>,
) {
    if skill_stack.is_activating || !skill_stack.is_ready() {
        return;
    }
    if config.input.skill1.just_pressed(&keyboard, &mouse)
        && let Some(skill) = player_data.skill_build.skills[1]
    {
        skill_stack.push(skill);
    }
    if config.input.skill2.just_pressed(&keyboard, &mouse)
        && let Some(skill) = player_data.skill_build.skills[2]
    {
        skill_stack.push(skill);
    }
    if config.input.skill3.just_pressed(&keyboard, &mouse)
        && let Some(skill) = player_data.skill_build.skills[3]
    {
        skill_stack.push(skill);
    }
    if config.input.skill4.just_pressed(&keyboard, &mouse)
        && let Some(skill) = player_data.skill_build.skills[4]
    {
        skill_stack.push(skill);
    }
}
