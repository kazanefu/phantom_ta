use bevy::prelude::*;

pub struct GameSystemSetPlugin;

impl Plugin for GameSystemSetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameSystemSet::Input,
                GameSystemSet::Detection,
                GameSystemSet::Logic,
                GameSystemSet::Rendering,
                GameSystemSet::Audio,
            )
                .chain(),
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameSystemSet {
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
