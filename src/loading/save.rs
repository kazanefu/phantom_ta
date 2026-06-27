use bevy::prelude::*;

use crate::loading::TaskKind;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SavingState>();
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum SavingState {
    #[default]
    Completed,
    Saving,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum SaveTaskKind {
    PlayerSaveDataList,
    Keymap,
}
impl super::TaskKind for SaveTaskKind {
    const TOTAL: usize = 2;
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
