use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct PlayerRestoreBundle {
    pub transform: Transform,
    pub visibility: Visibility,
    pub vel: Velocity,
    pub rigidbody: RigidBody,
    pub axis_lock: LockedAxes,
    pub gravity: GravityScale,
    pub friction: Friction,
    pub ccd: Ccd,
    pub collider: Collider,
    pub collision_group: CollisionGroups,
    pub jumping_timer: crate::player::JumpingTimer,
    pub down_state: crate::player::DownState,
    pub gate_spawn_immunity: crate::player::GateSpawnImmunity,
}

impl PlayerRestoreBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            visibility: Visibility::Visible,
            vel: Velocity::default(),
            rigidbody: RigidBody::Dynamic,
            axis_lock: LockedAxes::ROTATION_LOCKED,
            gravity: GravityScale(40.0),
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ccd: Ccd::enabled(),
            collider: Collider::capsule_y(20.0, 10.0),
            collision_group: CollisionGroups::new(crate::PLAYER_GROUP, Group::all()),
            jumping_timer: crate::player::JumpingTimer::default(),
            down_state: crate::player::DownState(false),
            gate_spawn_immunity: crate::player::GateSpawnImmunity,
        }
    }
}
