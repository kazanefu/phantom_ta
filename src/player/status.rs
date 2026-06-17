use bevy::prelude::*;

use crate::config::ControlConfig;

pub struct PlayerStatusPlugin;

impl Plugin for PlayerStatusPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct PlayerStatus {
    pub walk_speed: BaseRateStatus,
    pub jump_init_speed: BaseRateStatus,
    pub dash_speed: BaseRateStatus,
    pub run_speed: BaseRateStatus,
    pub attack: BaseRateStatus,
    pub diffence: BaseRateStatus,
    pub skill_cooltime_reduction: f32,
}

impl PlayerStatus {
    pub fn from_config(config: &ControlConfig) -> Self {
        Self {
            walk_speed: BaseRateStatus::from_base(config.base_walk_speed),
            jump_init_speed: BaseRateStatus::from_base(config.jump_init_vel),
            dash_speed: BaseRateStatus::from_base(config.dash_speed_rate),
            run_speed: BaseRateStatus::from_base(config.run_speed_rate),
            attack: BaseRateStatus::from_base(100.0),
            diffence: BaseRateStatus::from_base(0.0),
            skill_cooltime_reduction: 1.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct BaseRateStatus {
    pub base: f32,
    pub rate: f32,
}

impl BaseRateStatus {
    pub fn value(&self) -> f32 {
        self.base * self.rate
    }
    pub fn from_base(base: f32) -> Self {
        Self { base, rate: 1.0 }
    }
}
