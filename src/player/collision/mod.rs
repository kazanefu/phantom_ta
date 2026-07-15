use crate::game_system_set::GameSysSet;

use super::*;
mod one_way_platform;
mod gate;

pub struct PlayerCollisionPlugin;

impl Plugin for PlayerCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            one_way_platform::set_group_filter.in_set(GameSysSet::Detection),
        );
        app.add_plugins(gate::GateCollisionPlugin);
        app.add_systems(
            Update,
            clear_gate_spawn_immunity.in_set(GameSysSet::Rendering),
        );
    }
}

fn clear_gate_spawn_immunity(
    mut commands: Commands,
    que: Query<Entity, (Added<GateSpawnImmunity>, With<Player>)>,
) {
    for entity in &que {
        commands.entity(entity).remove::<GateSpawnImmunity>();
    }
}
