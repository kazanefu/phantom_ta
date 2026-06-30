use bevy::{prelude::*, render::render_resource::Face::Back};

use crate::{JpFont, loading::SavingState, menu::MenuState};
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (update_save_button, update_skill_building_button)
                    .run_if(in_state(MenuState::MainMenu)),
            );
    }
}

#[derive(Component)]
struct SaveButton;

fn main_menu_bundle() -> impl Bundle {
    (
        DespawnOnExit(MenuState::MainMenu),
        Node {
            width: percent(30.0),
            height: percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: px(10.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.8)),
    )
}

#[derive(Component)]
struct SkillBuildingButton;

fn save_button_bundle(font: Handle<Font>) -> impl Bundle {
    (
        Button,
        SaveButton,
        Node {
            width: percent(90.0),
            height: percent(20.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
    )
}
fn skill_building_button_bundle(font: Handle<Font>) -> impl Bundle {
    (
        Button,
        SkillBuildingButton,
        Node {
            width: percent(90.0),
            height: percent(20.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
    )
}

fn spawn_main_menu(mut commands: Commands, jp_font: Res<JpFont>) {
    let main_menu = commands.spawn(main_menu_bundle()).id();
    let save_button = commands
        .spawn(save_button_bundle(jp_font.font.clone()))
        .id();
    let skill_building_button = commands
        .spawn(skill_building_button_bundle(jp_font.font.clone()))
        .id();
    commands
        .entity(main_menu)
        .add_children(&[save_button, skill_building_button]);
}

fn update_save_button(
    mut que: Query<(&mut BackgroundColor, &Interaction), (With<SaveButton>, Changed<Interaction>)>,
    mut save_state: ResMut<NextState<SavingState>>,
) {
    for (mut color, interaction) in &mut que {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgba(0.1, 0.9, 0.2, 0.8));
                save_state.set(SavingState::Saving);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.8));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.8));
            }
        }
    }
}

fn update_skill_building_button(
    mut que: Query<
        (&mut BackgroundColor, &Interaction),
        (With<SkillBuildingButton>, Changed<Interaction>),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (mut color, interaction) in &mut que {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgba(0.1, 0.9, 0.2, 0.8));
                menu_state.set(MenuState::SkillBuilding);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.8));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.8));
            }
        }
    }
}
