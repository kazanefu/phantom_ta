use bevy::prelude::*;

use crate::{GameState, follow::Follower, player::Player};

pub struct CameraRoomStatePlugin;

impl Plugin for CameraRoomStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RoomTransition), deactivate_camera)
            .add_systems(OnEnter(GameState::Playing), activate_or_spawn_camera)
            .add_systems(
                Update,
                sync_camera_target.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct TransitionManagedCamera;

fn deactivate_camera(mut camera_que: Query<&mut Camera, With<TransitionManagedCamera>>) {
    for mut camera in &mut camera_que {
        camera.is_active = false;
    }
}

fn activate_or_spawn_camera(
    mut commands: Commands,
    mut camera_que: Query<(&mut Camera, &mut Follower), With<TransitionManagedCamera>>,
    player_que: Query<Entity, With<Player>>,
) {
    let player = player_que.single().ok();
    if let Ok((mut camera, mut follower)) = camera_que.single_mut() {
        camera.is_active = true;
        follower.target = player;
        return;
    }

    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Follower {
            target: player,
            follow_speed: 0.5,
        },
        TransitionManagedCamera,
    ));
}

fn sync_camera_target(
    player_que: Query<Entity, With<Player>>,
    mut camera_que: Query<&mut Follower, With<TransitionManagedCamera>>,
) {
    let Ok(player) = player_que.single() else {
        return;
    };
    for mut follower in &mut camera_que {
        follower.target = Some(player);
    }
}
