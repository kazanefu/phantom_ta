use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{game_system_set::GameSysSet, player::status::PlayerStatus, time::TimeState};

pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkillStack>()
            .add_message::<SkillActivateMsg>()
            .configure_sets(
                Update,
                (SkillSysSet::Activation, SkillSysSet::Execution)
                    .chain()
                    .run_if(in_state(TimeState::Running))
                    .in_set(GameSysSet::Logic),
            )
            .add_systems(
                Update,
                ((
                    tick_skill_cooldown,
                    activate_skill.run_if(|skill_stack: Res<SkillStack>| {
                        skill_stack.is_activating && skill_stack.is_ready()
                    }),
                )
                    .chain(),)
                    .in_set(SkillSysSet::Activation),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum SkillSysSet {
    Activation,
    Execution,
}

/// The player's skill build.
/// chosen by the player from a list of available skills
#[derive(Serialize, Deserialize, Clone)]
pub struct SkillBuild {
    pub skills: [Option<SkillKind>; Self::DEFAULT_CAPACITY],
}

impl SkillBuild {
    const DEFAULT_CAPACITY: usize = 4;
}
impl Default for SkillBuild {
    fn default() -> Self {
        Self {
            skills: [None, None, None, None],
        }
    }
}

/// chosen from the player's skill build.
/// Once activated, pop all skills from the stack.
#[derive(Resource)]
pub struct SkillStack {
    pub stack: Vec<SkillKind>,
    pub capacity: usize,
    pub is_activating: bool,
    cooldown: f32,
    pub index: usize,
}
impl SkillStack {
    const DEFAULT_CAPACITY: usize = 4;
    pub fn push(&mut self, skill: SkillKind) {
        if self.stack.len() < self.capacity && !self.is_activating {
            self.stack.push(skill);
            self.index = 0;
        }
    }
    pub fn pop(&mut self) -> Option<SkillKind> {
        let result = self.stack.pop();
        self.index += 1;
        if self.is_empty() {
            self.is_activating = false;
            self.index = 0;
        }
        result
    }
    pub fn pop_enumerate(&mut self) -> Option<(usize, SkillKind)> {
        let index = self.index;
        match self.pop() {
            Some(skill) => Some((index, skill)),
            None => None,
        }
    }
    pub fn activate(&mut self) {
        if !self.is_empty() {
            self.is_activating = true;
        }
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn is_ready(&self) -> bool {
        self.cooldown <= 0.0
    }
    pub fn set_cooldown(&mut self, cooldown: f32) {
        self.cooldown = self.cooldown.max(cooldown);
    }
}
impl Default for SkillStack {
    fn default() -> Self {
        Self {
            stack: Vec::with_capacity(Self::DEFAULT_CAPACITY),
            capacity: Self::DEFAULT_CAPACITY,
            is_activating: false,
            cooldown: 0.0,
            index: 0,
        }
    }
}

fn tick_skill_cooldown(
    player_que: Query<&PlayerStatus>,
    mut skill_stack: ResMut<SkillStack>,
    time: Res<Time>,
) {
    let Ok(player_status) = player_que.single() else {
        return;
    };
    if !skill_stack.is_ready() {
        skill_stack.cooldown -= time.delta_secs() * player_status.skill_cooltime_reduction;
    }
}

fn activate_skill(mut skill_stack: ResMut<SkillStack>, mut msg: MessageWriter<SkillActivateMsg>) {
    let Some((index, skill)) = skill_stack.pop_enumerate() else {
        return;
    };

    skill_stack.set_cooldown(skill.cooldown());

    for _ in 0..skill.parallel_count() {
        let Some((parallel_index, parallel_skill)) = skill_stack.pop_enumerate() else {
            break;
        };
        skill_stack.set_cooldown(parallel_skill.cooldown());
        msg.write(SkillActivateMsg::new(parallel_index, parallel_skill));
    }
    msg.write(SkillActivateMsg::new(index, skill));
}

#[derive(Message)]
pub struct SkillActivateMsg {
    pub skill_index: usize,
    pub skill_kind: SkillKind,
}
impl SkillActivateMsg {
    pub fn new(skill_index: usize, skill_kind: SkillKind) -> Self {
        Self {
            skill_index,
            skill_kind,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillKind {
    TestSkill,
    TestSkill2,
    UnconditionalJump,
    ParallelProcessing,
    NormalAttackBuff,
}

impl SkillKind {
    pub const NUM_SKILLS: usize = 5;
    pub fn cooldown(self) -> f32 {
        match self {
            Self::TestSkill => 1.0,
            Self::TestSkill2 => 2.0,
            Self::UnconditionalJump => 0.0,
            Self::ParallelProcessing => 0.0,
            Self::NormalAttackBuff => 0.0,
        }
    }
    pub fn parallel_count(self) -> usize {
        match self {
            Self::ParallelProcessing => 3,
            _ => 1,
        }
    }
    pub fn index(self) -> usize {
        self as usize
    }
}
