use bevy::prelude::*;
mod control;
mod input;
mod keymap_fs;

pub use control::ControlConfig;

use crate::file::SaveLoad;

pub struct PlayerConfigPlugin;

impl Plugin for PlayerConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .add_systems(Startup, load_input_setting);
    }
}

#[derive(Resource)]
pub struct PlayerConfig {
    pub input: input::InputSetting,
    pub control: control::ControlConfig,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            input: input::InputSetting::default(),
            control: control::ControlConfig::default(),
        }
    }
}

fn load_input_setting(mut res: ResMut<PlayerConfig>) {
    res.input = input::InputSetting::load_default_path().unwrap_or_default();
}
