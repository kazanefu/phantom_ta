use crate::player::skills::SkillKind;

use super::*;

mod test_attack;

pub struct NormalAttackPlugin;

impl Plugin for NormalAttackPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentNormalAttack>()
            .add_plugins(test_attack::TestAttackPlugin);
    }
}

#[derive(Resource)]
pub struct CurrentNormalAttack {
    pub kind: SkillKind,
}

impl Default for CurrentNormalAttack {
    fn default() -> Self {
        Self {
            kind: SkillKind::TestNormalAttack,
        }
    }
}
