use bevy::{anti_alias::taa, prelude::*};
mod control;
mod input;
mod keymap_fs;

pub use control::ControlConfig;

use crate::{GameState, file::SaveLoad, loading::LoadTaskState};

pub struct PlayerConfigPlugin;

impl Plugin for PlayerConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>().add_systems(
            Update,
            load_input_setting.run_if(in_state(GameState::Loading).and(
                |tasklist: Res<LoadTaskState>| {
                    !tasklist.is_task_done(crate::loading::LoadTaskKind::Keymap)
                },
            )),
        );
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

fn load_input_setting(mut res: ResMut<PlayerConfig>, mut tasklist: ResMut<LoadTaskState>) {
    res.input = input::InputSetting::load_default_path().unwrap_or_default();
    tasklist.set_task_done(crate::loading::LoadTaskKind::Keymap);
    println!("done load keymap");
}
