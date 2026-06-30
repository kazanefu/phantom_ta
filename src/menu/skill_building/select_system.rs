use super::*;

#[derive(Resource, Default)]
pub struct SelectedSelectedButton {
    pub id: Option<usize>,
}

#[derive(Message)]
pub struct ReloadSkillBuildMsg;

pub fn selected_pressed(
    mut button_que: Query<
        (&mut BackgroundColor, &Interaction, &mut SelectedSkillButton),
        Changed<Interaction>,
    >,
    mut player_data: ResMut<PlayerSaveData>,
    mut reload_msg: MessageWriter<ReloadSkillBuildMsg>,
    mut selected_button: ResMut<SelectedSelectedButton>,
) {
    for (mut color, interaction, mut button) in &mut button_que {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgba(0.1, 0.9, 0.2, 0.8));
                selected_button.id = Some(button.id);
                if button.skill.is_some() {
                    button.skill = None;
                }
                player_data.skill_build.skills[button.id] = button.skill;
                reload_msg.write(ReloadSkillBuildMsg);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 1.0));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.8));
            }
        }
    }
}

use std::fmt::Write;

pub fn reload_selected_skill_button(
    mut msg: MessageReader<ReloadSkillBuildMsg>,
    mut button_que: Query<(&Children, &mut SelectedSkillButton)>,
    mut text_que: Query<&mut Text, With<SelectedSkillButtonText>>,
    player_data: Res<PlayerSaveData>,
) {
    for _ in msg.read() {
        for (children, mut button) in &mut button_que {
            button.skill = player_data.skill_build.skills[button.id];
            for child in children.iter() {
                if let Ok(mut text) = text_que.get_mut(child) {
                    let skill_name: &'static str = match button.skill {
                        Some(skill) => skill.into(),
                        None => "None",
                    };
                    text.clear();
                    unsafe {
                        // Use `unwrap_unchecked` to avoid bounds checking for performance
                        // This is safe because write! will not panic when writing to a String
                        write!(text, "{}", skill_name).unwrap_unchecked();
                    }
                }
            }
        }
    }
}

pub fn candidate_pressed(
    mut button_que: Query<
        (&mut BackgroundColor, &Interaction, &CandidateSkillButton),
        Changed<Interaction>,
    >,
    selected_button: Res<SelectedSelectedButton>,
    mut player_data: ResMut<PlayerSaveData>,
    mut reload_msg: MessageWriter<ReloadSkillBuildMsg>,
) {
    for (mut color, interaction, button) in &mut button_que {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgba(0.1, 0.9, 0.2, 0.8));
                if let Some(selected_id) = selected_button.id {
                    player_data.skill_build.skills[selected_id] = Some(button.skill);
                    reload_msg.write(ReloadSkillBuildMsg);
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 1.0));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.8));
            }
        }
    }
}
