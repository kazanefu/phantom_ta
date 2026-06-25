use super::*;

#[derive(Resource)]
pub struct PlayerSaveDataList {
    pub list: Vec<PlayerSaveData>,
}

impl PlayerSaveDataList {
    pub fn set_id(&mut self) {
        for (i, data) in self.list.iter_mut().enumerate() {
            data.id = i;
        }
    }
}

#[derive(Resource)]
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
