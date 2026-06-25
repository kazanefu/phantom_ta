use bevy::prelude::*;

use crate::{config::PlayerConfig, game_system_set::GameSysSet};

pub struct EndAppPlugin;

impl Plugin for EndAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EndAppMsg>().add_systems(
            Update,
            (input_end_app, end_app_system)
                .chain()
                .in_set(GameSysSet::Input),
        );
    }
}

#[derive(Message)]
pub struct EndAppMsg;

fn end_app_system(mut msg: MessageReader<EndAppMsg>, mut exit: MessageWriter<AppExit>) {
    for _ in msg.read() {
        exit.write(AppExit::Success);
    }
}

fn input_end_app(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    config: Res<PlayerConfig>,
    mut msg: MessageWriter<EndAppMsg>,
) {
    if config.input.end_app.just_pressed(&keyboard, &mouse) {
        msg.write(EndAppMsg);
    }
}
