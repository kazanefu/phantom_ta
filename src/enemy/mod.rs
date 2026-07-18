use serde::{Deserialize, Serialize};
mod enemies;
use bevy::prelude::*;
mod range_detection;
use range_detection::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(range_detection::RangeDetectionPlugin);
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum EnemyKind {
    Normal,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Enemy {
    pub kind: EnemyKind,
    pub level: f32,
    pub pos: Vec2,
    pub range: DetectionRange,
}
