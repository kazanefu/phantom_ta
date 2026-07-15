use bevy::prelude::*;

use crate::{GameState, JpFont};

pub struct TransitionScreenPlugin;

impl Plugin for TransitionScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RoomTransition), spawn_transition_screen);
    }
}

fn spawn_transition_screen(mut commands: Commands, jp_font: Res<JpFont>) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1000.0),
        DespawnOnExit(GameState::RoomTransition),
    ));

    commands.spawn((
        Node {
            width: percent(100.0),
            height: percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        DespawnOnExit(GameState::RoomTransition),
        children![(
            Text::new("Room遷移中..."),
            TextFont {
                font: jp_font.font.clone(),
                font_size: 42.0,
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    ));
}
