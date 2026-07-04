use bevy::prelude::*;

pub mod collision;
pub mod file;
pub mod follow;
pub mod input_field;
pub mod scroll_ui;
pub mod tile;
pub mod time;

pub use input_field::*;

use crate::scroll_ui::ScrollUiPlugin;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(follow::FollowPlugins);
        app.add_plugins(input_field::InputFieldPlugin)
            .add_plugins(ScrollUiPlugin)
            .add_plugins(time::TimePlugin);
    }
}
