use bevy::prelude::*;

use crate::{GameState, JpFont};

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Title), setup_title)
            .add_systems(
                Update,
                update_start_button.run_if(in_state(GameState::Title)),
            );
    }
}

fn setup_title(mut commands: Commands, jp_font: Res<JpFont>) {
    println!("Title");
    // camera
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0),
        DespawnOnExit(GameState::Title),
    ));
    // background
    commands.spawn((
        Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(1920.0, 1080.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
        DespawnOnExit(GameState::Title),
    ));
    // transition to next state(SavaDataSelect) button
    commands.spawn((
        DespawnOnExit(GameState::Title),
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            StartButton,
            Button,
            Node {
                width: Val::Px(280.0),
                min_width: percent(20),
                min_height: Val::Px(72.0),
                padding: UiRect::axes(Val::Px(24.0), Val::Px(16.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.9, 0.2)),
            children![(
                Node {
                    max_width: percent(100),
                    ..default()
                },
                Text::new("スタート"),
                TextFont {
                    font: jp_font.font.clone(),
                    font_size: 32.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Center),
                TextColor(Color::srgb(0.2, 0.2, 0.2))
            )],
        )],
    ));
}

#[derive(Component)]
struct StartButton;

fn update_start_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&mut BackgroundColor, &Interaction),
        (Changed<Interaction>, With<StartButton>),
    >,
) {
    for (mut background, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Transition to the next state
                next_state.set(GameState::SaveDataSelect);
            }
            Interaction::Hovered => {
                *background = BackgroundColor(Color::WHITE);
            }
            Interaction::None => {
                *background = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
        }
    }
}
