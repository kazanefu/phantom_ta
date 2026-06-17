use bevy::prelude::*;

pub struct GameSystemSetPlugin;

impl Plugin for GameSystemSetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameSysSet::Input,
                GameSysSet::Detection,
                GameSysSet::Logic,
                GameSysSet::Rendering,
                GameSysSet::Audio,
            )
                .chain(),
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameSysSet {
    Input,
    // collision detection, HP monitoring, etc.
    Detection,
    // movement, update ui, animation, etc.
    Logic,
    // material update, visual effects, etc.
    Rendering,
    // sound effects, bgm, etc.
    Audio,
}
