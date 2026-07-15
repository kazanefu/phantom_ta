use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::PlayerBundle;

#[derive(Component)]
pub struct GateSpawnImmunity;

#[derive(Bundle)]
pub struct PlayerSpawnBundle {
    pub bundle: PlayerBundle,
    pub transform: Transform,
    pub collider: Collider,
    pub sprite: Sprite,
    pub gate_spawn_immunity: GateSpawnImmunity,
}

impl PlayerSpawnBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            bundle: PlayerBundle::default(),
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            collider: Collider::capsule_y(20.0, 10.0),
            sprite: Sprite {
                color: Color::srgb(0.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(20.0, 60.0)),
                ..default()
            },
            gate_spawn_immunity: GateSpawnImmunity,
        }
    }
}
