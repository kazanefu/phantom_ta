use super::*;
use crate::{
    config::PlayerConfig,
    game_system_set::GameSysSet,
    ground_state::GroundState,
    player::input_systems::{DashMsg, JumpMsg, MoveXInput},
};
use bevy_rapier2d::prelude::*;

pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_dash_cool_time, start_dash, update_dash_time)
                .chain()
                .in_set(GameSysSet::Detection),
        )
        .add_systems(
            Update,
            (update_x_velocity, jump_action).in_set(GameSysSet::Logic),
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
