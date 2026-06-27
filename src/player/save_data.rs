use crate::file::{Ron, SaveLoad};
use crate::loading::TaskState;
use crate::loading::{LoadTaskKind, LoadTaskState};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct PlayerSaveDataList {
    pub list: Vec<PlayerSaveData>,
}

pub fn load_player_save_data_list(
    mut res: ResMut<PlayerSaveDataList>,
    mut tasklist: ResMut<LoadTaskState>,
) {
    *res = PlayerSaveDataList::load_default_path().unwrap_or_default();
    res.set_id();
    tasklist.set_task_done(LoadTaskKind::PlayerSaveDataList);
    println!("done load player save data list");
}

impl PlayerSaveDataList {
    pub fn set_id(&mut self) {
        for (i, data) in self.list.iter_mut().enumerate() {
            data.id = i;
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct PlayerSaveData {
    pub id: usize,
    pub name: String,
    level: f32,
    available_skills: [bool; skills::SkillKind::NUM_SKILLS],
    skill_build: skills::SkillBuild,
    available_checkpoints: HashSet<usize>,
    last_checkpoint: usize,
}

impl Default for PlayerSaveData {
    fn default() -> Self {
        const AVAILABLE_SKILLS: [bool; skills::SkillKind::NUM_SKILLS] = {
            let mut temp = [false; skills::SkillKind::NUM_SKILLS];
            temp[0] = true;
            temp
        };
        let mut available_checkpoints = HashSet::new();
        available_checkpoints.insert(0);
        Self {
            id: 0,
            name: "new data".to_string(),
            level: 1.0,
            available_skills: AVAILABLE_SKILLS,
            skill_build: skills::SkillBuild::default(),
            available_checkpoints: available_checkpoints,
            last_checkpoint: 0,
        }
    }
}

impl SaveLoad for PlayerSaveDataList {
    const PATH: &'static str = "save_data/data.ron";
    type Format = Ron;
}

impl PlayerSaveDataList {
    pub fn push_new_data(&mut self, name: impl Into<String>) {
        let mut data = PlayerSaveData::default();
        data.name = name.into();
        data.id = self.list.len();
        self.list.push(data);
        self.save_default_path()
            .unwrap_or_else(|e| eprintln!("Failed to save player save data list: {}", e));
    }
}
