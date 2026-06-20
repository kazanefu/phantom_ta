use bevy::prelude::*;

use crate::game_system_set::GameSysSet;

pub struct FollowPlugins;

impl Plugin for FollowPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_system.in_set(GameSysSet::Logic));
    }
}

#[derive(Component)]
pub struct Follower {
    pub target: Option<Entity>,
    pub follow_speed: f32,
}

fn follow_system(
    time: Res<Time>,
    mut follower_que: Query<(&mut Transform, &Follower)>,
    target_que: Query<&GlobalTransform>,
) {
    for (mut transform, follower) in &mut follower_que {
        let Some(target_entity) = follower.target else {
            continue;
        };
        let Ok(target_transform) = target_que.get(target_entity) else {
            continue;
        };
        transform.translation = transform.translation.lerp(
            target_transform.translation(),
            follower.follow_speed * time.delta_secs(),
        );
    }
}
