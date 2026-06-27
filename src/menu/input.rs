use bevy::prelude::*;

use crate::{config::PlayerConfig, game_system_set::GameSysSet, menu::MenuState, time::TimeState};

pub struct MenuInputPlugin;

impl Plugin for MenuInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, switch_menu_input.in_set(GameSysSet::Input));
    }
}

fn switch_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    config: Res<PlayerConfig>,
    now_time_state: Res<State<TimeState>>,
    mut next_time_state: ResMut<NextState<TimeState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if !config.input.menu.just_pressed(&keyboard, &mouse) {
        return;
    }
    match now_time_state.get() {
        TimeState::Paused => {
            next_time_state.set(TimeState::Running);
            menu_state.set(MenuState::Closed);
        }
        TimeState::Running => {
            next_time_state.set(TimeState::Paused);
            menu_state.set(MenuState::MainMenu);
        }
    }
}
