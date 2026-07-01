use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum EnemyKind {
    Normal,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Enemy {
    pub kind: EnemyKind,
    pub level: f32,
}
