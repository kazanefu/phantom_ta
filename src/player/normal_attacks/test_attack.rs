use crate::{
    character::AttackHitboxBundle,
    player::{
        control_systems::NormalAttackMsg,
        input_systems::MousePosition,
        skills::{SkillActivateMsg, SkillStack},
    },
    time::TimeState,
};

use super::*;

pub struct TestAttackPlugin;

impl Plugin for TestAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            test_attack
                .run_if(|current_attack: Res<PlayerSaveData>| {
                    current_attack.skill_build.skills[0] == Some(SkillKind::TestNormalAttack)
                })
                .run_if(in_state(TimeState::Running)),
        );
    }
}

fn test_attack(
    mut commands: Commands,
    mut player_que: Query<(&Transform, &PlayerStatus), With<Player>>,
    mut msg: MessageReader<SkillActivateMsg>,
    mouse_pos: Res<MousePosition>,
    mut skill_stack: ResMut<SkillStack>,
) {
    for skill in msg.read() {
        if skill.skill_kind != SkillKind::TestNormalAttack {
            continue;
        }
        for (transform, status) in &mut player_que {
            skill_stack.set_cooldown(0.4);
            commands.spawn((
                AttackHitboxBundle::new(
                    40.0 * status.attack.value(),
                    Vec2::ZERO,
                    Some(CharacterKind::Player),
                    Collider::ball(10.0),
                ),
                Transform::from_translation(
                    transform.translation + mouse_pos.direction.extend(0.0) * 20.0,
                ),
                RigidBody::Dynamic,
                Velocity::linear(mouse_pos.direction * 1000.0),
            ));
        }
    }
}
