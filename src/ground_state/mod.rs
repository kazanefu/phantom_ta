use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bitflags::bitflags;
use smallvec::SmallVec;

use crate::game_system_set::GameSysSet;

mod systems;

pub struct GroundStatePlugin;

impl Plugin for GroundStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            systems::update_ground_state.in_set(GameSysSet::Detection),
        );
    }
}

bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy, Default)]
    pub struct ContactFlag: u8 {
        const OnGround = 0b00000001;
        const OnWall = 0b00000010;
        const OnCeiling = 0b00000100;
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Component, Default)]
pub struct GroundState {
    pub contact_flag: ContactFlag,
    pub contacts: SmallVec<[ContactInfo; 4]>,
    pub platform: Option<Entity>,
}

impl GroundState {
    pub fn is_grounded(&self) -> bool {
        self.contact_flag.contains(ContactFlag::OnGround)
    }

    pub fn on_wall(&self) -> bool {
        self.contact_flag.contains(ContactFlag::OnWall)
    }

    pub fn on_ceiling(&self) -> bool {
        self.contact_flag.contains(ContactFlag::OnCeiling)
    }
    pub fn normal(&self) -> Vec2 {
        self.contacts
            .iter()
            .map(|c| c.normal)
            .sum::<Vec2>()
            .normalize_or(Vec2::Y)
    }
    pub fn normal_ground_filtered(&self) -> Vec2 {
        self.contacts
            .iter()
            .map(|c| c.normal)
            .filter(|v| v.y >= 0.7)
            .sum::<Vec2>()
            .normalize_or(Vec2::Y)
    }
}

#[derive(Clone, Copy)]
pub struct ContactInfo {
    pub entity: Entity,
    pub normal: Vec2,
}
