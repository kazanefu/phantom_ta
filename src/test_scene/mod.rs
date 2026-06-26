use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    follow::Follower, ground_state::Ground, player::PlayerBundle, InputField, InputFieldBundle,
    JpFont, ONE_WAY_PLATFORM_GROUP,
};

pub struct TestScenePlugin;

impl Plugin for TestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands, jp_font: Res<JpFont>) {
    commands.spawn((
        RigidBody::Fixed,
        Transform::from_xyz(0.0, -30.0, 0.0).with_rotation(Quat::from_rotation_z(0.0)),
        Collider::cuboid(1000.0, 10.0),
        Ground,
        Sprite {
            color: Color::srgb(0.5, 0.8, 0.0),
            custom_size: Some(Vec2::new(2000.0, 20.0)),
            ..default()
        },
    ));
    commands.spawn((
        RigidBody::Fixed,
        Transform::from_xyz(0.0, -100.0, 0.0).with_rotation(Quat::from_rotation_z(PI / 4.0)),
        Collider::cuboid(1000.0, 10.0),
        Ground,
        Sprite {
            color: Color::srgb(0.5, 0.8, 0.0),
            custom_size: Some(Vec2::new(2000.0, 20.0)),
            ..default()
        },
        CollisionGroups::new(ONE_WAY_PLATFORM_GROUP, Group::all()),
    ));
    commands.spawn((
        RigidBody::Fixed,
        Transform::from_xyz(40.0, -900.0, 0.0),
        Collider::cuboid(10.0, 1000.0),
        Ground,
        Sprite {
            color: Color::srgb(0.5, 0.8, 0.0),
            custom_size: Some(Vec2::new(20.0, 2000.0)),
            ..default()
        },
    ));
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

    commands.spawn(InputFieldBundle {
        input_field: InputField::new("ここに入力"),
        button: Button,
        node: Node {
            position_type: PositionType::Absolute,
            top: px(12),
            right: px(12),
            width: px(320),
            height: px(36),
            padding: UiRect::axes(px(8), px(6)),
            border: UiRect::all(px(2)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(px(6)),
            ..default()
        },
        background_color: BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
        border_color: BorderColor::all(Color::srgb(0.35, 0.35, 0.35)),
        text: Text::new(""),
        text_font: TextFont {
            font: jp_font.font.clone(),
            font_size: 20.0,
            ..default()
        },
        text_color: TextColor(Color::WHITE),
        text_layout: TextLayout::new_with_justify(Justify::Left),
    });
}
