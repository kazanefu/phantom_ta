use bevy::prelude::*;

use crate::{GameState, JpFont, game_system_set::GameSysSet, player::skills::SkillStack};

pub struct PlayerSkillUiPlugin;

impl Plugin for PlayerSkillUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_skill_ui)
            .add_systems(Update, skill_stack_ui_update.in_set(GameSysSet::Rendering));
    }
}

#[derive(Bundle)]
struct SkillUiCanvas {
    node: Node,
}
impl Default for SkillUiCanvas {
    fn default() -> Self {
        Self {
            node: Node {
                width: percent(100.0),
                height: percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: percent(1.0),
                column_gap: percent(1.0),
                ..default()
            },
        }
    }
}

#[derive(Component)]
struct SkillStackUi {
    id: usize,
}

fn skill_stack_ui_canvas() -> impl Bundle {
    (
        Node {
            width: percent(100.0),
            height: percent(7.0),
            flex_direction: FlexDirection::Row,
            row_gap: percent(1.0),
            column_gap: percent(1.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        DespawnOnExit(GameState::Playing),
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.1)),
    )
}

fn skill_stack_ui(id: usize, font: Handle<Font>) -> impl Bundle {
    (
        SkillStackUi { id },
        Node {
            width: percent(7.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.8)),
        Visibility::Visible,
        children![(
            Text::new((id + 1).to_string()),
            TextFont {
                font_size: 10.0,
                font,
                ..default()
            }
        )],
    )
}

fn spawn_skill_ui(mut commands: Commands, stack: Res<SkillStack>, font: Res<JpFont>) {
    let base = commands.spawn(SkillUiCanvas::default()).id();
    let canvas = commands.spawn(skill_stack_ui_canvas()).id();
    for i in 0..stack.capacity {
        let skill_ui = commands.spawn(skill_stack_ui(i, font.font.clone())).id();
        commands.entity(canvas).add_child(skill_ui);
    }
    commands.entity(base).add_child(canvas);
}

fn skill_stack_ui_update(
    mut que: Query<(&mut BackgroundColor, &SkillStackUi)>,
    stack: Res<SkillStack>,
) {
    for (mut color, ui) in &mut que {
        if stack.stack.len() > ui.id {
            color.0 = Color::srgba(1.0, 1.0, 1.0, 1.0);
        } else {
            color.0 = Color::srgba(0.5, 0.5, 0.5, 0.8);
        }
    }
}
