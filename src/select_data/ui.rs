use crate::{
    InputField, InputFieldBundle, JpFont,
    player::{PlayerSaveData, PlayerSaveDataList},
    scroll_ui::ScrollUi,
};

use super::*;

#[derive(Component)]
pub struct AddDataButton {
    name_entity: Entity,
}
fn add_data_button_bundle(font: Handle<Font>, name_entity: Entity) -> impl Bundle {
    (
        Button,
        AddDataButton { name_entity },
        Node {
            width: percent(30.0),
            height: percent(5.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.55, 0.55, 0.55)),
        children![(
            Text::new("Add new data"),
            TextFont {
                font,
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    )
}

pub fn pressed_add_data(
    mut commands: Commands,
    current_canvas: Res<CurrentCanvas>,
    mut data_list: ResMut<PlayerSaveDataList>,
    input_field_que: Query<&InputField>,
    mut button_que: Query<(&AddDataButton, &Interaction), (Changed<Interaction>, With<Button>)>,
    font: Res<JpFont>,
) {
    if current_canvas.entity.is_none() {
        return;
    }
    for (button, interaction) in &mut button_que {
        if *interaction == Interaction::Pressed {
            if let Ok(input_field) = input_field_que.get(button.name_entity) {
                let name = input_field.value.clone();
                data_list.push_new_data(name);
                let button = commands
                    .spawn(save_data_button(
                        font.font.clone(),
                        data_list.list.last().unwrap_or(&PlayerSaveData::default()),
                    ))
                    .id();
                commands
                    .entity(current_canvas.entity.unwrap())
                    .add_child(button);
            }
        }
    }
}

#[derive(Component)]
pub struct SaveDataSelectButton {
    data_id: usize,
}
fn save_data_button(font: Handle<Font>, data: &PlayerSaveData) -> impl Bundle {
    (
        Button,
        SaveDataSelectButton { data_id: data.id },
        Node {
            width: percent(30.0),
            height: percent(5.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        children![(
            Text::new(format!("{}", data.name)),
            TextFont {
                font,
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    )
}

pub fn pressed_save_data_button(
    mut next_state: ResMut<NextState<GameState>>,
    data_list: Res<PlayerSaveDataList>,
    mut current_data: ResMut<PlayerSaveData>,
    mut button_que: Query<
        (&SaveDataSelectButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (buton, interaction, mut background) in &mut button_que {
        match *interaction {
            Interaction::Pressed => {
                if let Some(data) = data_list.list.get(buton.data_id) {
                    *current_data = data.clone();
                    next_state.set(GameState::RoomTransition);
                }
            }
            Interaction::Hovered => {
                *background = BackgroundColor(Color::WHITE);
            }
            Interaction::None => {
                *background = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct CurrentCanvas {
    pub entity: Option<Entity>,
}

pub fn spawn_ui(
    mut commands: Commands,
    font: Res<JpFont>,
    mut data_list: ResMut<PlayerSaveDataList>,
    mut current_canvas: ResMut<CurrentCanvas>,
) {
    *current_canvas = CurrentCanvas { entity: None };
    data_list.set_id();
    let scroll_ui = ScrollUi::new(&mut commands, 100.0, 100.0);
    let name_input_field = commands
        .spawn(InputFieldBundle::new_center(
            "Input name here...",
            font.font.clone(),
        ))
        .id();
    let add_data_button = commands
        .spawn(add_data_button_bundle(font.font.clone(), name_input_field))
        .id();
    scroll_ui.add_child(&mut commands, name_input_field);
    scroll_ui.add_child(&mut commands, add_data_button);
    for data in &data_list.list {
        let button = commands
            .spawn(save_data_button(font.font.clone(), data))
            .id();
        scroll_ui.add_child(&mut commands, button);
    }
    commands
        .entity(scroll_ui.base)
        .insert(DespawnOnExit(GameState::SaveDataSelect));
    current_canvas.entity = Some(scroll_ui.sub);
}
