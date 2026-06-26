use super::Settings;
use crate::file::SaveLoad;

pub fn get_settings() -> Settings {
    Settings::load_default_path().unwrap_or_else(|err| {
        eprintln!("Failed to load settings: {err}");
        Settings::default()
    })
}