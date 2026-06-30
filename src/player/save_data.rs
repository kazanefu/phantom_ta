use crate::file::{Ron, SaveLoad};
use crate::loading::{LoadTaskKind, LoadTaskState, SaveTaskKind};
use crate::loading::{SaveTaskState, TaskState};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

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

pub fn save_player_save_data_list(
    mut res: ResMut<PlayerSaveDataList>,
    current_data: Res<PlayerSaveData>,
    mut tasklist: ResMut<SaveTaskState>,
) {
    res.list
        .iter_mut()
        .find(|data| data.id == current_data.id)
        .map(|data| *data = current_data.clone());
    res.save_default_path().unwrap_or_else(|e| {
        eprintln!("Failed to save player save data list: {}", e);
        return;
    });
    tasklist.set_task_done(SaveTaskKind::PlayerSaveDataList);
    println!("done save player save data list");
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
    pub level: f32,
    pub available_skills: [bool; skills::SkillKind::NUM_SKILLS],
    pub skill_build: skills::SkillBuild,
    pub available_checkpoints: HashSet<usize>,
    pub last_checkpoint: usize,
}

impl Default for PlayerSaveData {
    fn default() -> Self {
        const AVAILABLE_SKILLS: [bool; skills::SkillKind::NUM_SKILLS] = {
            let mut temp = [false; skills::SkillKind::NUM_SKILLS];
            temp[skills::SkillKind::DEFAULT_NORMAL_ATTACK.index()] = true;
            temp[skills::SkillKind::DEFAULT_SKILL.index()] = true;
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

impl PlayerSaveData {
    pub const MAX_EXP: f32 = 10000.0;
    /// Level = 1.0 + 99.0 * sqrt(amount_exp / MAX_EXP) , 1 <= Level <= 100
    pub fn level_up(&mut self, exp: f32) -> f32 {
        let mut amount_exp = Self::MAX_EXP * ((self.level - 1.0) / 99.0).powi(2);
        amount_exp += exp;
        if amount_exp > Self::MAX_EXP {
            amount_exp = Self::MAX_EXP;
        }
        self.level = 1.0 + 99.0 * (amount_exp / Self::MAX_EXP).sqrt();
        self.level
    }
    pub fn available_skills(&self) -> impl Iterator<Item = skills::SkillKind> {
        skills::SkillKind::iter().filter(|skill| self.available_skills[skill.index()])
    }
}
