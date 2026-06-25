use crate::config::player::input::InputKey::Key;

use super::keymap_fs::{
    keycode_to_str, mousebutton_to_str, string_to_keycode, string_to_mousebutton,
};
use bevy::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy)]
pub enum InputKey {
    Key(KeyCode),
    Mouse(MouseButton),
}

impl InputKey {
    fn just_pressed(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        match *self {
            Self::Key(key_code) => keyboard.just_pressed(key_code),
            Self::Mouse(mouse_button) => mouse.just_pressed(mouse_button),
        }
    }
    fn pressed(&self, keyboard: &ButtonInput<KeyCode>, mouse: &ButtonInput<MouseButton>) -> bool {
        match *self {
            Self::Key(key_code) => keyboard.pressed(key_code),
            Self::Mouse(mouse_button) => mouse.pressed(mouse_button),
        }
    }
    fn just_released(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        match *self {
            Self::Key(key_code) => keyboard.just_released(key_code),
            Self::Mouse(mouse_button) => mouse.just_released(mouse_button),
        }
    }
}

#[derive(Clone, Copy)]
pub struct KeyBindings {
    pub keys: [Option<InputKey>; Self::MAX_KEYS],
}

impl KeyBindings {
    pub const MAX_KEYS: usize = 4;
    pub const fn new(keys: [Option<InputKey>; Self::MAX_KEYS]) -> Self {
        Self { keys }
    }

    pub fn just_pressed(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        self.keys
            .iter()
            .flatten()
            .any(|key| key.just_pressed(keyboard, mouse))
    }
    pub fn pressed(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        self.keys
            .iter()
            .flatten()
            .any(|key| key.pressed(keyboard, mouse))
    }
    pub fn just_released(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        self.keys
            .iter()
            .flatten()
            .any(|key| key.just_released(keyboard, mouse))
    }
}

impl Serialize for KeyBindings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.keys.len()))?;
        for key in &self.keys {
            seq.serialize_element(&input_key_to_str(*key))?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for KeyBindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let items: Vec<String> = Vec::deserialize(deserializer)?;
        let mut keys = [None; Self::MAX_KEYS];
        for (i, item) in items.iter().take(Self::MAX_KEYS).enumerate() {
            keys[i] = str_to_input_key(item);
        }
        Ok(KeyBindings::new(keys))
    }
}

fn input_key_to_str(key: Option<InputKey>) -> String {
    match key {
        Some(InputKey::Key(code)) => keycode_to_str(&code),
        Some(InputKey::Mouse(button)) => mousebutton_to_str(&button),
        None => "None".to_string(),
    }
}

fn str_to_input_key(s: &str) -> Option<InputKey> {
    if s == "None" {
        None
    } else if let Some(code) = string_to_keycode(s) {
        Some(InputKey::Key(code))
    } else if let Some(button) = string_to_mousebutton(s) {
        Some(InputKey::Mouse(button))
    } else {
        None
    }
}

#[derive(Serialize, Deserialize)]
pub struct InputSetting {
    pub jump: KeyBindings,
    pub left: KeyBindings,
    pub right: KeyBindings,
    pub down: KeyBindings,
    pub dash: KeyBindings,
    pub attack: KeyBindings,
    pub end_app: KeyBindings,
}

impl Default for InputSetting {
    fn default() -> Self {
        Self {
            jump: KeyBindings::new([
                Some(InputKey::Key(KeyCode::Space)),
                Some(InputKey::Key(KeyCode::ArrowUp)),
                Some(InputKey::Key(KeyCode::KeyW)),
                None,
            ]),
            left: KeyBindings::new([
                Some(InputKey::Key(KeyCode::ArrowLeft)),
                Some(InputKey::Key(KeyCode::KeyA)),
                None,
                None,
            ]),
            right: KeyBindings::new([
                Some(InputKey::Key(KeyCode::ArrowRight)),
                Some(InputKey::Key(KeyCode::KeyD)),
                None,
                None,
            ]),
            down: KeyBindings::new([
                Some(InputKey::Key(KeyCode::ArrowDown)),
                Some(InputKey::Key(KeyCode::KeyS)),
                None,
                None,
            ]),
            dash: KeyBindings::new([
                Some(InputKey::Key(KeyCode::ShiftLeft)),
                Some(InputKey::Mouse(MouseButton::Right)),
                None,
                None,
            ]),
            attack: KeyBindings::new([Some(InputKey::Mouse(MouseButton::Left)), None, None, None]),
            end_app: KeyBindings::new([Some(Key(KeyCode::Backspace)), None, None, None]),
        }
    }
}
