use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

const NORMAL_BACKGROUND: Color = Color::srgb(0.12, 0.12, 0.12);
const FOCUSED_BACKGROUND: Color = Color::srgb(0.06, 0.06, 0.06);
const NORMAL_BORDER: Color = Color::srgb(0.35, 0.35, 0.35);
const FOCUSED_BORDER: Color = Color::srgb(0.35, 0.7, 1.0);
const TEXT_COLOR: Color = Color::WHITE;
const PLACEHOLDER_COLOR: Color = Color::srgb(0.55, 0.55, 0.55);

pub struct InputFieldPlugin;

impl Plugin for InputFieldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FocusedInputField>().add_systems(
            Update,
            (
                clear_focused_input_field,
                set_focused_input_field,
                update_background_color,
                show_text_or_placeholder,
                recieve_text_input,
            )
                .chain(),
        );
    }
}

#[derive(Resource, Default)]
struct FocusedInputField(Option<Entity>);

#[derive(Component, Debug, Clone, Default)]
pub struct InputField {
    pub value: String,
    pub placeholder: String,
    pub multiline: bool,
    pub max_length: Option<usize>,
    pub selected_background_color: Color,
    pub unselected_background_color: Color,
    pub value_text_color: Color,
    pub placeholder_text_color: Color,
    pub selected_border_color: Color,
    pub unselected_border_color: Color,
}

impl InputField {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            value: String::new(),
            placeholder: placeholder.into(),
            multiline: false,
            selected_background_color: FOCUSED_BACKGROUND,
            unselected_background_color: NORMAL_BACKGROUND,
            value_text_color: TEXT_COLOR,
            selected_border_color: FOCUSED_BORDER,
            unselected_border_color: NORMAL_BORDER,
            placeholder_text_color: PLACEHOLDER_COLOR,
            max_length: None,
        }
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

fn clear_focused_input_field(
    mut focused: ResMut<FocusedInputField>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        focused.0 = None;
    }
}
// after clear_focused
fn set_focused_input_field(
    mut focused_input_field: ResMut<FocusedInputField>,
    mut interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (entity, interaction) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                focused_input_field.0 = Some(entity);
            }
            _ => {}
        }
    }
}

fn update_background_color(
    mut query: Query<(Entity, &mut BackgroundColor, &mut BorderColor, &InputField)>,
    focused_input_field: Res<FocusedInputField>,
) {
    for (entity, mut background_color, mut border_color, input_field) in &mut query {
        if Some(entity) == focused_input_field.0 {
            *background_color = BackgroundColor(input_field.selected_background_color);
            *border_color = BorderColor::all(input_field.selected_border_color);
        } else {
            *background_color = BackgroundColor(input_field.unselected_background_color);
            *border_color = BorderColor::all(input_field.unselected_border_color);
        }
    }
}

fn show_text_or_placeholder(
    mut query: Query<(Entity, &InputField, &mut Text, &mut TextColor)>,
    focused_input_field: Res<FocusedInputField>,
) {
    use std::fmt::Write;
    for (entity, input_field, mut text, mut text_color) in &mut query {
        let contents: &str =
            if input_field.value.is_empty() && Some(entity) != focused_input_field.0 {
                text_color.0 = input_field.placeholder_text_color;
                &input_field.placeholder
            } else {
                text_color.0 = input_field.value_text_color;
                &input_field.value
            };
        text.clear();
        unsafe {
            // Use `unwrap_unchecked` to avoid bounds checking for performance
            // This is safe because write! will not panic when writing to a String
            write!(text, "{}", contents).unwrap_unchecked();
        }
    }
}

fn recieve_text_input(
    mut events: MessageReader<KeyboardInput>,
    mut que: Query<&mut InputField>,
    focused_input_field: Res<FocusedInputField>,
) {
    let Some(focused_entity) = focused_input_field.0 else {
        return;
    };
    let Ok(mut input_field) = que.get_mut(focused_entity) else {
        return;
    };
    for event in events.read() {
        if !event.state.is_pressed() {
            continue;
        }
        match &event.logical_key {
            Key::Backspace => {
                input_field.backspace();
            }
            Key::Enter => {
                if input_field.multiline {
                    input_field.insert_text("\n");
                }
            }
            Key::Escape => {
                // Do nothing for now
            }
            _ => {
                if let Some(c) = &event.text {
                    input_field.insert_text(c);
                }
            }
        }
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
