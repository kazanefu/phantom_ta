use super::*;

mod test_attack;

pub struct NormalAttackPlugin;

impl Plugin for NormalAttackPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentNormalAttack>()
            .add_plugins(test_attack::TestAttackPlugin);
    }
}

#[derive(Resource, Default)]
pub struct CurrentNormalAttack {
    pub kind: NormalAttackKind,
}

#[repr(u8)]
#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum NormalAttackKind {
    #[default]
    TestAttack,
    TestAttack2,
}
