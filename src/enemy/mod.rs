use serde::{Deserialize, Serialize};
mod enemies;
use bevy::prelude::*;
mod range_detection;
mod spawn;
use range_detection::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(range_detection::RangeDetectionPlugin)
            .add_plugins(spawn::EnemySpawnPlugin);
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub enum EnemyKind {
    #[default]
    Normal,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct EnemyInfo {
    pub kind: EnemyKind,
    pub level: f32,
    pub pos: Vec2,
    pub range: DetectionRange,
}
