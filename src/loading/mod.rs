use bevy::prelude::*;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadTaskState>()
            .add_systems(OnEnter(GameState::Loading), reset_loading_task_state)
            .add_systems(
                Update,
                check_complete_loading_task.run_if(in_state(GameState::Loading)),
            );
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LoadTaskKind {
    PlayerSaveDataList,
    Keymap,
}

impl LoadTaskKind {
    const TOTAL: usize = 2;
}

#[derive(Resource, Default)]
pub struct LoadTaskState {
    tasks: [bool; LoadTaskKind::TOTAL],
}

impl LoadTaskState {
    pub fn set_task_done(&mut self, kind: LoadTaskKind) {
        self.tasks[kind as usize] = true;
    }

    pub fn is_all_done(&self) -> bool {
        self.tasks.iter().all(|&done| done)
    }

    pub fn clear(&mut self) {
        self.tasks = [false; LoadTaskKind::TOTAL];
    }

    pub fn is_task_done(&self, kind: LoadTaskKind) -> bool {
        self.tasks[kind as usize]
    }
}

fn check_complete_loading_task(
    load_task_state: ResMut<LoadTaskState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if load_task_state.is_all_done() {
        next_state.set(GameState::Start);
        println!("All loading tasks are completed. Transitioning to Start state.");
        return;
    }
    // println!("Loading...");
}

fn reset_loading_task_state(mut load_task_state: ResMut<LoadTaskState>) {
    load_task_state.clear();
}
