use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::EnumCount;

use crate::loading::TaskKind;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SavingState>()
            .init_resource::<SaveTaskState>()
            .add_systems(OnEnter(SavingState::Saving), reset_saving_task_state)
            .add_systems(
                Update,
                check_complete_saving_task.run_if(in_state(SavingState::Saving)),
            );
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum SavingState {
    #[default]
    Completed,
    Saving,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, EnumCount)]
pub enum SaveTaskKind {
    PlayerSaveDataList,
    Keymap,
}
impl super::TaskKind for SaveTaskKind {
    const TOTAL: usize = Self::COUNT;
}

#[derive(Resource, Default)]
pub struct SaveTaskState {
    tasks: [bool; SaveTaskKind::TOTAL],
}

impl super::TaskState for SaveTaskState {
    type Kind = SaveTaskKind;
    fn set_task_done(&mut self, kind: Self::Kind) {
        self.tasks[kind as usize] = true;
    }
    fn clear(&mut self) {
        self.tasks = [false; Self::Kind::TOTAL];
    }
    fn is_all_done(&self) -> bool {
        self.tasks.iter().all(|&done| done)
    }
    fn is_task_done(&self, kind: Self::Kind) -> bool {
        self.tasks[kind as usize]
    }
}

use super::TaskState;

fn reset_saving_task_state(mut tasklist: ResMut<SaveTaskState>) {
    tasklist.clear();
}

fn check_complete_saving_task(
    tasklist: Res<SaveTaskState>,
    mut state: ResMut<NextState<SavingState>>,
) {
    if tasklist.is_all_done() {
        state.set(SavingState::Completed);
    }
}
