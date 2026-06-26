use bevy::prelude::*;

pub mod collision;
pub mod file;
pub mod follow;
pub mod input_field;

pub use input_field::*;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(follow::FollowPlugins);
        app.add_plugins(input_field::InputFieldPlugin);
    }
}
