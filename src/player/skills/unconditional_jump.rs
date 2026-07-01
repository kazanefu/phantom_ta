use crate::player::{JumpingTimer, Player, control_systems::PlayerAction};

use super::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct UnconditionalJumpPlugin;

impl Plugin for UnconditionalJumpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            unconditional_jump_action
                .in_set(GameSysSet::Logic)
                .in_set(PlayerAction::Last),
        );
    }
}

fn unconditional_jump_action(
    mut que: Query<(&mut Velocity, &mut JumpingTimer, &PlayerStatus), With<Player>>,
    mut msg: MessageReader<SkillActivateMsg>,
) {
    for m in msg.read() {
        if m.skill_kind != SkillKind::UnconditionalJump {
            continue;
        }
        for (_velocity, mut jumping_timer, _status) in &mut que {
            jumping_timer.jumping_time = 0.5;
            jumping_timer.hold_time = 0.5;
        }
    }
    for (mut velocity, jumping_timer, status) in &mut que {
        if jumping_timer.jumping_time > 0.0 {
            velocity.linear.y =
                status.jump_init_speed.value() * (jumping_timer.hold_time + 1.0).clamp(1.0, 3.3);
        }
    }
}
