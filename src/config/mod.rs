use bevy::prelude::*;
mod player;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerConfigPlugin);
    }
}

pub use player::{ControlConfig, PlayerConfig};
