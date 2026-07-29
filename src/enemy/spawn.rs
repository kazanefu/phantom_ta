use bevy::{ecs::system::command, prelude::*};

use crate::{
    enemy::{EnemyInfo, range_detection::DetectionRange},
    game_system_set::GameSysSet,
};

pub struct EnemySpawnPlugin;
impl Plugin for EnemySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnEnemyMsg>()
            .add_systems(Update, spawn_enemy.in_set(GameSysSet::Logic));
    }
}

#[derive(Component)]
pub struct EnemySpawner {
    pub enemy_info: crate::enemy::EnemyInfo,
}

#[derive(Bundle)]
struct EnemySpawnerBundle {
    spawner: EnemySpawner,
    transform: Transform,
    range: DetectionRange,
}

#[derive(Message)]
pub struct SpawnEnemyMsg {
    pub enemy_info: EnemyInfo,
}

impl EnemySpawnerBundle {
    pub fn new(enemy_info: EnemyInfo) -> Self {
        Self {
            spawner: EnemySpawner { enemy_info },
            transform: Transform::from_translation(enemy_info.pos.extend(0.0)),
            range: enemy_info.range,
        }
    }
}

// spawns enemies when the player enters the detection range of an enemy spawner
// once the spawner spawns an enemy, it is despawned
fn spawn_enemy(
    mut commands: Commands,
    spawner_que: Query<(Entity, &EnemySpawner, &DetectionRange)>,
    mut msg: MessageWriter<SpawnEnemyMsg>,
) {
    for (entity, spawner, range) in &spawner_que {
        if !range.is_inside() {
            continue;
        }
        msg.write(SpawnEnemyMsg {
            enemy_info: spawner.enemy_info,
        });
        commands.entity(entity).despawn();
    }
}
