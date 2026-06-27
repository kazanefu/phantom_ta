use bevy::prelude::*;
mod ui;
use crate::{GameState, select_data::ui::CurrentCanvas};

pub struct SelectDataPlugin;

impl Plugin for SelectDataPlugin {
    fn build(&self, app: &mut App) {
        // Plugin implementation
        app.init_resource::<CurrentCanvas>()
            .add_systems(
                OnEnter(GameState::SaveDataSelect),
                (setup_select_data, ui::spawn_ui),
            )
            .add_systems(
                Update,
                ui::pressed_add_data.run_if(in_state(GameState::SaveDataSelect)),
            );
    }
}

fn setup_select_data(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::default(),
        DespawnOnExit(GameState::SaveDataSelect),
    ));
}
