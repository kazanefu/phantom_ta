use super::input::InputSetting;
use bevy::prelude::*;

impl InputSetting {
    pub fn load_from_file(path: &str) -> Self {
        match std::fs::read_to_string(path) {
            Ok(contents) => {
                if let Ok(setting) = ron::from_str::<InputSetting>(&contents) {
                    return setting;
                }
            }
            Err(_) => {}
        }

        let default = Self::default();
        let _ = default.save_to_file(path);
        default
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        let ron_str = ron::ser::to_string_pretty(self, Default::default())?;
        std::fs::write(path, ron_str)?;
        Ok(())
    }
}

const KEYCODE_MAP: &[(&str, KeyCode)] = &[
    ("Space", KeyCode::Space),
    ("Quote", KeyCode::Quote),
    ("Comma", KeyCode::Comma),
    ("Minus", KeyCode::Minus),
    ("Period", KeyCode::Period),
    ("Slash", KeyCode::Slash),
    ("Digit0", KeyCode::Digit0),
    ("Digit1", KeyCode::Digit1),
    ("Digit2", KeyCode::Digit2),
    ("Digit3", KeyCode::Digit3),
    ("Digit4", KeyCode::Digit4),
    ("Digit5", KeyCode::Digit5),
    ("Digit6", KeyCode::Digit6),
    ("Digit7", KeyCode::Digit7),
    ("Digit8", KeyCode::Digit8),
    ("Digit9", KeyCode::Digit9),
    ("Semicolon", KeyCode::Semicolon),
    ("Equal", KeyCode::Equal),
    ("BracketLeft", KeyCode::BracketLeft),
    ("Backslash", KeyCode::Backslash),
    ("BracketRight", KeyCode::BracketRight),
    ("Backquote", KeyCode::Backquote),
    ("KeyA", KeyCode::KeyA),
    ("KeyB", KeyCode::KeyB),
    ("KeyC", KeyCode::KeyC),
    ("KeyD", KeyCode::KeyD),
    ("KeyE", KeyCode::KeyE),
    ("KeyF", KeyCode::KeyF),
    ("KeyG", KeyCode::KeyG),
    ("KeyH", KeyCode::KeyH),
    ("KeyI", KeyCode::KeyI),
    ("KeyJ", KeyCode::KeyJ),
    ("KeyK", KeyCode::KeyK),
    ("KeyL", KeyCode::KeyL),
    ("KeyM", KeyCode::KeyM),
    ("KeyN", KeyCode::KeyN),
    ("KeyO", KeyCode::KeyO),
    ("KeyP", KeyCode::KeyP),
    ("KeyQ", KeyCode::KeyQ),
    ("KeyR", KeyCode::KeyR),
    ("KeyS", KeyCode::KeyS),
    ("KeyT", KeyCode::KeyT),
    ("KeyU", KeyCode::KeyU),
    ("KeyV", KeyCode::KeyV),
    ("KeyW", KeyCode::KeyW),
    ("KeyX", KeyCode::KeyX),
    ("KeyY", KeyCode::KeyY),
    ("KeyZ", KeyCode::KeyZ),
    ("Escape", KeyCode::Escape),
    ("Enter", KeyCode::Enter),
    ("Tab", KeyCode::Tab),
    ("Backspace", KeyCode::Backspace),
    ("Insert", KeyCode::Insert),
    ("Delete", KeyCode::Delete),
    ("ArrowRight", KeyCode::ArrowRight),
    ("ArrowLeft", KeyCode::ArrowLeft),
    ("ArrowDown", KeyCode::ArrowDown),
    ("ArrowUp", KeyCode::ArrowUp),
    ("PageUp", KeyCode::PageUp),
    ("PageDown", KeyCode::PageDown),
    ("Home", KeyCode::Home),
    ("End", KeyCode::End),
    ("CapsLock", KeyCode::CapsLock),
    ("ScrollLock", KeyCode::ScrollLock),
    ("NumLock", KeyCode::NumLock),
    ("PrintScreen", KeyCode::PrintScreen),
    ("Pause", KeyCode::Pause),
    ("NumpadDivide", KeyCode::NumpadDivide),
    ("NumpadMultiply", KeyCode::NumpadMultiply),
    ("NumpadSubtract", KeyCode::NumpadSubtract),
    ("NumpadAdd", KeyCode::NumpadAdd),
    ("NumpadEnter", KeyCode::NumpadEnter),
    ("Numpad0", KeyCode::Numpad0),
    ("Numpad1", KeyCode::Numpad1),
    ("Numpad2", KeyCode::Numpad2),
    ("Numpad3", KeyCode::Numpad3),
    ("Numpad4", KeyCode::Numpad4),
    ("Numpad5", KeyCode::Numpad5),
    ("Numpad6", KeyCode::Numpad6),
    ("Numpad7", KeyCode::Numpad7),
    ("Numpad8", KeyCode::Numpad8),
    ("Numpad9", KeyCode::Numpad9),
    ("NumpadDecimal", KeyCode::NumpadDecimal),
    ("NumpadEqual", KeyCode::NumpadEqual),
    ("F1", KeyCode::F1),
    ("F2", KeyCode::F2),
    ("F3", KeyCode::F3),
    ("F4", KeyCode::F4),
    ("F5", KeyCode::F5),
    ("F6", KeyCode::F6),
    ("F7", KeyCode::F7),
    ("F8", KeyCode::F8),
    ("F9", KeyCode::F9),
    ("F10", KeyCode::F10),
    ("F11", KeyCode::F11),
    ("F12", KeyCode::F12),
    ("ShiftLeft", KeyCode::ShiftLeft),
    ("ShiftRight", KeyCode::ShiftRight),
    ("ControlLeft", KeyCode::ControlLeft),
    ("ControlRight", KeyCode::ControlRight),
    ("AltLeft", KeyCode::AltLeft),
    ("AltRight", KeyCode::AltRight),
    ("SuperLeft", KeyCode::SuperLeft),
    ("SuperRight", KeyCode::SuperRight),
];

pub fn keycode_to_str(code: &KeyCode) -> String {
    KEYCODE_MAP
        .iter()
        .find(|(_, key)| key == code)
        .map(|(s, _)| s.to_string())
        .unwrap_or_else(|| format!("{:?}", code))
}

pub fn string_to_keycode(s: &str) -> Option<KeyCode> {
    KEYCODE_MAP
        .iter()
        .find(|(name, _)| *name == s)
        .map(|(_, code)| *code)
}

const MOUSEBUTTON_MAP: &[(&str, MouseButton)] = &[
    ("Left", MouseButton::Left),
    ("Right", MouseButton::Right),
    ("Middle", MouseButton::Middle),
    ("Back", MouseButton::Back),
    ("Forward", MouseButton::Forward),
];

pub fn mousebutton_to_str(button: &MouseButton) -> String {
    MOUSEBUTTON_MAP
        .iter()
        .find(|(_, b)| b == button)
        .map(|(s, _)| s.to_string())
        .unwrap_or_else(|| format!("{:?}", button))
}

pub fn string_to_mousebutton(s: &str) -> Option<MouseButton> {
    MOUSEBUTTON_MAP
        .iter()
        .find(|(name, _)| *name == s)
        .map(|(_, button)| *button)
}
