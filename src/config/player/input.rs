use bevy::prelude::*;

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
}

#[derive(Clone, Copy)]
pub struct KeyBindings {
    keys: [Option<InputKey>; Self::MAX_KEYS],
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
}

#[derive(Clone, Copy)]
pub struct InputSetting {
    pub jump: KeyBindings,
    pub left: KeyBindings,
    pub right: KeyBindings,
    pub down: KeyBindings,
    pub dash: KeyBindings,
    pub attack: KeyBindings,
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
        }
    }
}
