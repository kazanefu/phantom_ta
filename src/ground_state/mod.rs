use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bitflags::bitflags;
use smallvec::SmallVec;

mod systems;

pub struct GroundStatePlugin;

impl Plugin for GroundStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::update_ground_state);
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
}

#[derive(Clone, Copy)]
pub struct ContactInfo {
    pub entity: Entity,
    pub normal: Vec2,
}
