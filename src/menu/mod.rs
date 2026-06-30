use bevy::prelude::*;

mod input;
mod main_menu;
mod skill_building;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_plugins(input::MenuInputPlugin)
            .add_plugins(main_menu::MainMenuPlugin)
            .add_plugins(skill_building::SkillBuildingPlugin);
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum MenuState {
    #[default]
    Closed,
    MainMenu,
    SkillBuilding,
    Status,
}
