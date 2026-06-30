use bevy::prelude::*;

use crate::{
    JpFont,
    menu::MenuState,
    player::{PlayerSaveData, skills::SkillKind},
};

pub struct SkillBuildingPlugin;

impl Plugin for SkillBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::SkillBuilding), spawn_skill_building);
    }
}

fn skill_building_base_canvas() -> impl Bundle {
    (
        Node {
            width: percent(50.0),
            height: percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
    )
}

fn skill_building_selected_skill_canvas() -> impl Bundle {
    (
        Node {
            width: percent(90.0),
            height: percent(30.0),
            flex_direction: FlexDirection::Row,
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.7)),
    )
}

fn skill_building_candidate_skills_canvas() -> impl Bundle {
    (
        Node {
            width: percent(90.0),
            height: percent(70.0),
            flex_direction: FlexDirection::Row,
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.7)),
    )
}

#[derive(Component)]
struct SelectedSkillButton {
    id: usize, // 0..4
    skill: Option<SkillKind>,
}
fn selected_skill_button(
    id: usize,
    font: Handle<Font>,
    current_skill: Option<SkillKind>,
) -> impl Bundle {
    (
        SelectedSkillButton {
            id,
            skill: current_skill,
        },
        Button,
        Node {
            width: percent(10.0),
            height: percent(10.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
        children![(
            Node {
                max_height: percent(100.0),
                max_width: percent(100.0),
                ..default()
            },
            Text::new("None"),
            TextFont {
                font,
                font_size: 20.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
        )],
    )
}

#[derive(Component)]
struct CandidateSkillButton {
    skill: SkillKind,
}

fn candidate_skill_button(skill: SkillKind, font: Handle<Font>) -> impl Bundle {
    let skill_name: &'static str = skill.into();
    (
        CandidateSkillButton { skill },
        Button,
        Node {
            width: percent(10.0),
            height: percent(10.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
        children![(
            Node {
                max_height: percent(100.0),
                max_width: percent(100.0),
                ..default()
            },
            Text::new(skill_name),
            TextFont {
                font,
                font_size: 20.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
        )],
    )
}

fn spawn_skill_building(mut commands: Commands, font: Res<JpFont>, save_data: Res<PlayerSaveData>) {
    let base_canvas = commands.spawn(skill_building_base_canvas()).id();
    let selected_skill_canvas = commands.spawn(skill_building_selected_skill_canvas()).id();
    let candidate_skills_canvas = commands
        .spawn(skill_building_candidate_skills_canvas())
        .id();
    for skill in save_data.available_skills() {
        let button = commands
            .spawn(candidate_skill_button(skill, font.font.clone()))
            .id();
        commands.entity(candidate_skills_canvas).add_child(button);
    }
    for (i, skill) in save_data.skill_build.skills.iter().enumerate() {
        let button = commands
            .spawn(selected_skill_button(i, font.font.clone(), *skill))
            .id();
        commands.entity(selected_skill_canvas).add_child(button);
    }
    commands
        .entity(base_canvas)
        .add_children(&[selected_skill_canvas, candidate_skills_canvas])
        .insert(DespawnOnExit(MenuState::SkillBuilding));
}
