use bevy::prelude::*;

mod save;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadTaskState>()
            .add_systems(OnEnter(GameState::Loading), reset_loading_task_state)
            .add_systems(
                Update,
                (check_jp_font_loaded, check_complete_loading_task)
                    .run_if(in_state(GameState::Loading)),
            );
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LoadTaskKind {
    PlayerSaveDataList,
    Keymap,
    JpFont,
}

impl LoadTaskKind {
    const TOTAL: usize = 3;
}

#[derive(Resource, Default)]
pub struct LoadTaskState {
    tasks: [bool; LoadTaskKind::TOTAL],
}

impl TaskKind for LoadTaskKind {
    const TOTAL: usize = LoadTaskKind::TOTAL;
}

impl TaskState for LoadTaskState {
    type Kind = LoadTaskKind;
    fn set_task_done(&mut self, kind: LoadTaskKind) {
        self.tasks[kind as usize] = true;
    }

    fn is_all_done(&self) -> bool {
        self.tasks.iter().all(|&done| done)
    }

    fn clear(&mut self) {
        self.tasks = [false; LoadTaskKind::TOTAL];
    }

    fn is_task_done(&self, kind: LoadTaskKind) -> bool {
        self.tasks[kind as usize]
    }
}

#[derive(Resource, Clone, Default)]
pub struct JpFont {
    pub font: Handle<Font>,
}

fn check_complete_loading_task(
    load_task_state: ResMut<LoadTaskState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if load_task_state.is_all_done() {
        next_state.set(GameState::Title);
        println!("All loading tasks are completed. Transitioning to Title state.");
        return;
    }
    // println!("Loading...");
}

fn reset_loading_task_state(mut load_task_state: ResMut<LoadTaskState>) {
    load_task_state.clear();
}

fn check_jp_font_loaded(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut load_task_state: ResMut<LoadTaskState>,
    jp_font: Option<Res<JpFont>>,
) {
    if jp_font.is_none() {
        let font = asset_server.load("embedded://phantom_ta/fonts/NotoSansJP-Bold.ttf");
        commands.insert_resource(JpFont { font });
        return;
    }

    let Some(jp_font) = jp_font else {
        return;
    };

    if asset_server
        .get_recursive_dependency_load_state(&jp_font.font)
        .is_some_and(|state| state.is_loaded())
    {
        load_task_state.set_task_done(LoadTaskKind::JpFont);
    }
}

pub trait TaskState {
    type Kind: TaskKind;
    fn set_task_done(&mut self, kind: Self::Kind);
    fn is_all_done(&self) -> bool;
    fn clear(&mut self);
    fn is_task_done(&self, kind: Self::Kind) -> bool;
}

pub trait TaskKind {
    const TOTAL: usize;
}
