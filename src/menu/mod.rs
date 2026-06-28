use bevy::prelude::*;

mod input;
mod main_menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_plugins(input::MenuInputPlugin)
            .add_plugins(main_menu::MainMenuPlugin);
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum MenuState {
    #[default]
    Closed,
    MainMenu,
    NormalAttackSelect,
    SkillBuilding,
    Status,
}
