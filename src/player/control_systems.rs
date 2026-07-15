use super::*;
use crate::{
    config::PlayerConfig,
    game_system_set::GameSysSet,
    ground_state::GroundState,
    player::{
        input_systems::{AttackMsg, DashMsg, DownInput, JumpMsg, MoveXInput},
        skills::{SkillActivateMsg, SkillKind, SkillStack},
    },
    time::TimeState,
    GameState,
};

pub struct PlayerControlPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerAction {
    Base,
    Skill,
    Last,
}

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<NormalAttackMsg>()
            .configure_sets(
                Update,
                (PlayerAction::Base, PlayerAction::Skill, PlayerAction::Last).chain(),
            )
            .add_systems(
                Update,
                (update_dash_cool_time, start_dash, update_dash_time)
                    .chain()
                    .in_set(GameSysSet::Detection),
            )
            .add_systems(
                Update,
                (
                    ((
                        update_x_velocity,
                        update_jumping_timer,
                        jump_action,
                        down_action,
                        call_attack,
                    )
                        .chain())
                    .run_if(|stack: Res<SkillStack>| stack.is_ready() && !stack.is_activating)
                    .in_set(GameSysSet::Logic)
                    .in_set(PlayerAction::Base),
                    stop_while_cooldown
                        .run_if(|stack: Res<SkillStack>| {
                            !(stack.is_ready() && !stack.is_activating)
                        })
                        .in_set(GameSysSet::Logic)
                        .in_set(PlayerAction::Base),
                )
                    .run_if(in_state(TimeState::Running))
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn update_dash_cool_time(
    mut que: Query<&mut DashCoolTime>,
    time: Res<Time>,
    config: Res<PlayerConfig>,
) {
    let dt = time.delta_secs();
    for mut dash_ct in &mut que {
        if dash_ct.stock >= 2 {
            continue;
        }
        dash_ct.cool_time += dt;
        if dash_ct.cool_time >= config.control.dash_cooltime {
            dash_ct.stock += 1;
            dash_ct.cool_time = 0.0;
        }
    }
}

fn start_dash(
    mut que: Query<&mut DashCoolTime>,
    mut msg: MessageReader<DashMsg>,
    config: Res<PlayerConfig>,
) {
    for _ in msg.read() {
        for mut dash_ct in &mut que {
            if dash_ct.stock >= 1 {
                dash_ct.dash_time = config.control.dash_time;
                dash_ct.stock -= 1;
            }
        }
    }
}

fn update_dash_time(mut que: Query<&mut DashCoolTime>, time: Res<Time>) {
    let dt = time.delta_secs();
    for mut dash_ct in &mut que {
        if dash_ct.dash_time >= 0.0 {
            dash_ct.dash_time -= dt;
        }
    }
}

fn update_x_velocity(
    mut que: Query<(&mut Velocity, &PlayerStatus, &DashCoolTime, &GroundState), With<Player>>,
    input: Res<MoveXInput>,
) {
    for (mut velocity, status, dash_ct, ground_state) in &mut que {
        let normal = ground_state.normal_ground_filtered();
        let tangent = -normal.perp().normalize();
        let normal_vel = velocity.linear.dot(normal);
        let mut next_vel = tangent * input.direction * status.walk_speed.value();
        if dash_ct.dash_time > 0.0 {
            next_vel *= status.dash_speed.value();
        } else if input.is_running {
            next_vel *= status.run_speed.value();
        }
        velocity.linear = next_vel + normal * normal_vel;
    }
}

fn stop_while_cooldown(mut que: Query<(&mut Velocity), With<Player>>) {
    for (mut velocity) in &mut que {
        velocity.linear = Vec2::ZERO;
    }
}

fn update_jumping_timer(mut que: Query<&mut JumpingTimer>, time: Res<Time>) {
    for mut q in &mut que {
        if q.jumping_time >= 0.0 {
            q.jumping_time -= time.delta_secs();
        }
    }
}

pub fn jump_action(
    mut que: Query<
        (
            &mut Velocity,
            &mut JumpingTimer,
            &PlayerStatus,
            &GroundState,
        ),
        With<Player>,
    >,
    mut msg: MessageReader<JumpMsg>,
) {
    for JumpMsg(hold_time) in msg.read() {
        for (_velocity, mut jumping_timer, _status, ground_state) in &mut que {
            if !ground_state.is_grounded() {
                continue;
            }
            jumping_timer.jumping_time = 0.05;
            jumping_timer.hold_time = *hold_time;
        }
    }
    for (mut velocity, jumping_timer, status, ground_state) in &mut que {
        if !ground_state.is_grounded() {
            continue;
        }

        if jumping_timer.jumping_time > 0.0 {
            velocity.linear.y =
                status.jump_init_speed.value() * (jumping_timer.hold_time + 1.0).clamp(1.0, 3.3);
        }
    }
}

fn down_action(mut que: Query<&mut DownState, With<Player>>, input: Res<DownInput>) {
    for mut down_state in &mut que {
        down_state.0 = input.0;
    }
}

#[derive(Message)]
pub struct NormalAttackMsg;

/// Call attack action, if skill stack is not empty, activate skill, otherwise send normal attack message.
fn call_attack(
    mut stack: ResMut<SkillStack>,
    mut msg: MessageReader<AttackMsg>,
    mut skill_activate_msg: MessageWriter<SkillActivateMsg>,
    save_data: Res<PlayerSaveData>,
) {
    for &m in msg.read() {
        if m != AttackMsg::Start {
            continue;
        }
        if !stack.is_empty() {
            stack.activate();
        } else {
            skill_activate_msg.write(SkillActivateMsg {
                skill_index: 0,
                skill_kind: save_data.skill_build.skills[0]
                    .unwrap_or(SkillKind::DEFAULT_NORMAL_ATTACK),
            });
        }
    }
}
