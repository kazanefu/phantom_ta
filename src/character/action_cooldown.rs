use crate::game_system_set::GameSysSet;

use super::*;

pub struct ActionCooldownPlugin;

impl Plugin for ActionCooldownPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_action_cooldown.in_set(GameSysSet::Detection));
    }
}

#[derive(Component, Default)]
pub struct ActionCooldown {
    pub cooldown: f32,
}

impl ActionCooldown {
    pub fn is_ready(&self) -> bool {
        self.cooldown <= 0.0
    }
}

fn tick_action_cooldown(mut que: Query<&mut ActionCooldown>, time: Res<Time>) {
    let delta_secs = time.delta_secs();
    for mut cooldown in &mut que {
        if cooldown.is_ready() {
            continue;
        }
        cooldown.cooldown -= delta_secs;
    }
}
