use bevy::prelude::*;

pub mod collision;
pub mod file;
pub mod follow;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(follow::FollowPlugins);
    }
}
