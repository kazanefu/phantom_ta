use super::*;

#[derive(Resource, Default)]
pub struct PlayerSaveData {
    level: f32,
    available_skills: [bool; skills::SkillKind::NUM_SKILLS],
    skill_build: skills::SkillBuild,
    available_checkpoints: HashSet<usize>,
}
