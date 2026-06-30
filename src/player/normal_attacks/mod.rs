use crate::player::skills::SkillKind;

use super::*;

mod test_attack;

pub struct NormalAttackPlugin;

impl Plugin for NormalAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(test_attack::TestAttackPlugin);
    }
}
