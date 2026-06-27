use bevy::prelude::*;

pub fn pause(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

pub fn unpause(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum TimeState {
    #[default]
    Running,
    Paused,
}
