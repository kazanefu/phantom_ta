use crate::game_system_set::GameSysSet;

use super::*;
mod one_way_platform;

pub struct PlayerCollisionPlugin;

impl Plugin for PlayerCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            one_way_platform::set_group_filter.in_set(GameSysSet::Detection),
        );
    }
}
