use bevy::prelude::*;
mod input;

pub struct PlayerConfigPlugin;

impl Plugin for PlayerConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>();
    }
}

#[derive(Resource, Default)]
pub struct PlayerConfig {
    pub input: input::InputSetting,
}
