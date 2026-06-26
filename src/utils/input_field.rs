use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

const NORMAL_BACKGROUND: Color = Color::srgb(0.12, 0.12, 0.12);
const HOVERED_BACKGROUND: Color = Color::srgb(0.18, 0.18, 0.18);
const FOCUSED_BACKGROUND: Color = Color::srgb(0.16, 0.16, 0.16);
const NORMAL_BORDER: Color = Color::srgb(0.35, 0.35, 0.35);
const FOCUSED_BORDER: Color = Color::srgb(0.35, 0.7, 1.0);
const TEXT_COLOR: Color = Color::WHITE;
const PLACEHOLDER_COLOR: Color = Color::srgb(0.55, 0.55, 0.55);

pub struct InputFieldPlugin;

impl Plugin for InputFieldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FocusedInputField>()
            .add_message::<InputFieldSubmitted>()
            .add_systems(
                Update,
                (
                    validate_focused_input_field,
                    handle_input_field_interaction,
                    handle_input_field_ime_events,
                    handle_input_field_keyboard_input,
                    sync_input_field_visuals,
                )
                    .chain(),
            );
    }
}

#[derive(Resource, Default)]
struct FocusedInputField(Option<Entity>);

#[derive(Component, Debug, Clone)]
pub struct InputField {
    pub value: String,
    pub placeholder: String,
    pub preedit: String,
    pub multiline: bool,
    pub max_length: Option<usize>,
    pub ime_active: bool,
}

impl InputField {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            value: String::new(),
            placeholder: placeholder.into(),
            preedit: String::new(),
            multiline: false,
            max_length: None,
            ime_active: false,
        }
    }

    fn visible_text(&self, focused: bool) -> String {
        if self.value.is_empty() && self.preedit.is_empty() {
            if focused {
                "▏".to_string()
            } else {
                self.placeholder.clone()
            }
        } else {
            let mut text = String::with_capacity(self.value.len() + self.preedit.len() + 1);
            text.push_str(&self.value);
            text.push_str(&self.preedit);
            if focused {
                text.push('▏');
            }
            text
        }
    }

    fn clear_preedit(&mut self) {
        self.preedit.clear();
    }

    fn set_preedit(&mut self, value: &str) {
        self.preedit.clear();
        self.preedit.push_str(value);
    }

    fn insert_text(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        let remaining = self
            .max_length
            .map(|max_length| max_length.saturating_sub(self.value.chars().count()));
        match remaining {
            Some(0) => {}
            Some(remaining) => {
                self.value.extend(text.chars().take(remaining));
            }
            None => {
                self.value.push_str(text);
            }
        }
    }

    fn backspace(&mut self) {
        self.value.pop();
    }
}

#[derive(Bundle)]
pub struct InputFieldBundle {
    pub input_field: InputField,
    pub button: Button,
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub text: Text,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_layout: TextLayout,
}

impl InputFieldBundle {
    pub fn new(placeholder: impl Into<String>, font: Handle<Font>) -> Self {
        Self {
            input_field: InputField::new(placeholder),
            button: Button,
            node: Node {
                width: px(260),
                height: px(36),
                padding: UiRect::axes(px(8), px(6)),
                border: UiRect::all(px(2)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(px(6)),
                ..default()
            },
            background_color: BackgroundColor(NORMAL_BACKGROUND),
            border_color: BorderColor::all(NORMAL_BORDER),
            text: Text::new(""),
            text_font: TextFont {
                font,
                font_size: 20.0,
                ..default()
            },
            text_color: TextColor(TEXT_COLOR),
            text_layout: TextLayout::new_with_justify(Justify::Left),
        }
    }
}

#[derive(Message, Debug, Clone)]
pub struct InputFieldSubmitted {
    pub entity: Entity,
    pub value: String,
}

fn validate_focused_input_field(
    mut focused: ResMut<FocusedInputField>,
    mut windows: Single<&mut Window>,
    field_query: Query<&InputField>,
) {
    let Some(entity) = focused.0 else {
        return;
    };

    if field_query.get(entity).is_ok() {
        return;
    }

    focused.0 = None;
    windows.ime_enabled = false;
}

fn handle_input_field_interaction(
    mut focused: ResMut<FocusedInputField>,
    mut windows: Single<&mut Window>,
    mut field_query: Query<(Entity, &Interaction, &mut InputField), Changed<Interaction>>,
) {
    for (entity, interaction, mut field) in &mut field_query {
        if !matches!(*interaction, Interaction::Pressed) {
            continue;
        }

        focused.0 = Some(entity);
        field.ime_active = true;
        let ime_position = windows.cursor_position().unwrap_or_default();
        windows.ime_position = ime_position;
        windows.ime_enabled = true;
    }
}

fn handle_input_field_ime_events(
    mut ime_reader: MessageReader<Ime>,
    focused: Res<FocusedInputField>,
    mut field_query: Query<&mut InputField>,
    mut windows: Single<&mut Window>,
) {
    let Some(focused_entity) = focused.0 else {
        return;
    };

    let Ok(mut field) = field_query.get_mut(focused_entity) else {
        return;
    };

    for ime in ime_reader.read() {
        match ime {
            Ime::Preedit { value, cursor, .. } => {
                if cursor.is_none() {
                    field.clear_preedit();
                } else {
                    field.set_preedit(value);
                }
            }
            Ime::Commit { value, .. } => {
                field.clear_preedit();
                field.insert_text(value);
            }
            Ime::Enabled { .. } => {
                field.ime_active = true;
                windows.ime_enabled = true;
            }
            Ime::Disabled { .. } => {
                field.ime_active = false;
                field.clear_preedit();
                windows.ime_enabled = false;
            }
        }
    }
}

fn handle_input_field_keyboard_input(
    mut keyboard_reader: MessageReader<KeyboardInput>,
    mut focused: ResMut<FocusedInputField>,
    mut field_query: Query<&mut InputField>,
    mut submit_writer: MessageWriter<InputFieldSubmitted>,
    mut windows: Single<&mut Window>,
) {
    let Some(focused_entity) = focused.0 else {
        return;
    };

    let Ok(mut field) = field_query.get_mut(focused_entity) else {
        return;
    };

    for keyboard_input in keyboard_reader.read() {
        if !keyboard_input.state.is_pressed() {
            continue;
        }

        match (&keyboard_input.logical_key, &keyboard_input.text) {
            (Key::Escape, _) => {
                field.ime_active = false;
                field.clear_preedit();
                focused.0 = None;
                windows.ime_enabled = false;
            }
            (Key::Backspace, _) if !field.ime_active => {
                field.backspace();
            }
            (Key::Enter, _) if !field.ime_active => {
                if field.multiline {
                    field.insert_text("\n");
                } else {
                    submit_writer.write(InputFieldSubmitted {
                        entity: focused_entity,
                        value: field.value.clone(),
                    });
                }
            }
            (_, Some(inserted_text)) if !field.ime_active => {
                if inserted_text.chars().all(is_printable_char) {
                    field.insert_text(inserted_text);
                }
            }
            _ => {}
        }
    }
}

fn sync_input_field_visuals(
    focused: Res<FocusedInputField>,
    mut field_query: Query<(
        Entity,
        &mut InputField,
        &Interaction,
        &mut Text,
        &mut TextColor,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
) {
    for (entity, mut field, interaction, mut text, mut text_color, mut background, mut border_color)
        in &mut field_query
    {
        let is_focused = focused.0 == Some(entity);
        let display_text = field.visible_text(is_focused);

        field.ime_active = is_focused;
        if !is_focused {
            field.clear_preedit();
        }

        **text = display_text;
        text_color.0 = if is_focused || !field.value.is_empty() {
            TEXT_COLOR
        } else {
            PLACEHOLDER_COLOR
        };
        background.0 = if is_focused {
            FOCUSED_BACKGROUND
        } else {
            match *interaction {
                Interaction::Hovered => HOVERED_BACKGROUND,
                _ => NORMAL_BACKGROUND,
            }
        };
        *border_color = BorderColor::all(if is_focused {
            FOCUSED_BORDER
        } else {
            NORMAL_BORDER
        });
    }
}

fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
        || ('\u{f0000}'..='\u{ffffd}').contains(&chr)
        || ('\u{100000}'..='\u{10fffd}').contains(&chr);

    !is_in_private_use_area && !chr.is_ascii_control()
}