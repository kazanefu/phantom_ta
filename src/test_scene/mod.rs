use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    GameState, InputField, InputFieldBundle, JpFont, ONE_WAY_PLATFORM_GROUP,
    follow::Follower,
    ground_state::Ground,
    player::PlayerBundle,
    rooms::{ItemKind, RoomItem, SpawnItemMsg},
};

pub struct TestScenePlugin;

impl Plugin for TestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_scene);
    }
}

#[allow(dead_code)]
fn setup_ground(mut msg: MessageWriter<SpawnItemMsg>) {
    msg.write(SpawnItemMsg(RoomItem {
        kind: ItemKind::Ground,
        transform: Transform {
            translation: Vec3 {
                x: -300.0,
                y: -200.0,
                z: 0.0,
            },
            rotation: Quat::from_rotation_z(-PI / 4.0),
            scale: Vec3::new(1000.0, 300.0, 1.0),
        },
    }));
}

fn setup_scene(mut commands: Commands) {
    // commands.spawn((
    //     RigidBody::Fixed,
    //     Transform::from_xyz(0.0, -30.0, 0.0).with_rotation(Quat::from_rotation_z(0.0)),
    //     Collider::cuboid(1000.0, 10.0),
    //     Ground,
    //     Sprite {
    //         color: Color::srgb(0.5, 0.8, 0.0),
    //         custom_size: Some(Vec2::new(2000.0, 20.0)),
    //         ..default()
    //     },
    // ));
    // commands.spawn((
    //     RigidBody::Fixed,
    //     Transform::from_xyz(0.0, -100.0, 0.0).with_rotation(Quat::from_rotation_z(PI / 4.0)),
    //     Collider::cuboid(1000.0, 10.0),
    //     Ground,
    //     Sprite {
    //         color: Color::srgb(0.5, 0.8, 0.0),
    //         custom_size: Some(Vec2::new(2000.0, 20.0)),
    //         ..default()
    //     },
    //     CollisionGroups::new(ONE_WAY_PLATFORM_GROUP, Group::all()),
    // ));
    // commands.spawn((
    //     RigidBody::Fixed,
    //     Transform::from_xyz(40.0, -900.0, 0.0),
    //     Collider::cuboid(10.0, 1000.0),
    //     Ground,
    //     Sprite {
    //         color: Color::srgb(0.5, 0.8, 0.0),
    //         custom_size: Some(Vec2::new(20.0, 2000.0)),
    //         ..default()
    //     },
    // ));
    let player = commands
        .spawn((
            PlayerBundle::default(),
            Transform::from_xyz(0.0, 100.0, 0.0),
            Collider::capsule_y(20.0, 10.0),
            Sprite {
                color: Color::srgb(0.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(20.0, 60.0)),
                ..default()
            },
        ))
        .id();
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Follower {
            target: Some(player),
            follow_speed: 0.5,
        },
    ));
}
