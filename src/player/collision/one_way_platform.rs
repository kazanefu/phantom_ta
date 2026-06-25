use bevy::prelude::*;
use bevy_rapier2d::{dynamics::Velocity, geometry::CollisionGroups};

use crate::{
    ONE_WAY_PLATFORM_GROUP, character::Character, ground_state::GroundState, player::DownState,
};

pub fn set_group_filter(
    mut que: Query<(&mut CollisionGroups, &Velocity, &DownState, &GroundState), With<Character>>,
) {
    for (mut groups, velocity, down_state, ground_state) in &mut que {
        if (velocity.linear.y > 0.0 && !ground_state.is_grounded()) || down_state.0 {
            groups.filters.remove(ONE_WAY_PLATFORM_GROUP);
        } else {
            groups.filters.insert(ONE_WAY_PLATFORM_GROUP);
        }
    }
}
