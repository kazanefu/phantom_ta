use crate::{ATTACK_GROUP, collision::get_contained_entity, game_system_set::GameSysSet};

use super::*;
use bevy_rapier2d::prelude::*;
use smallvec::SmallVec;

pub struct AttackHitPlugin;

impl Plugin for AttackHitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hit_detection.in_set(GameSysSet::Detection));
    }
}

#[derive(Bundle)]
pub struct AttackHitboxBundle {
    pub attack_hitbox: AttackHitbox,
    pub collider: Collider,
    pub collision_group: CollisionGroups,
}

impl AttackHitboxBundle {
    pub fn new(
        damage: f32,
        knockback: Vec2,
        owner: Option<CharacterKind>,
        collider: Collider,
    ) -> Self {
        Self {
            attack_hitbox: AttackHitbox {
                damage,
                knockback,
                owner,
                hit_list: SmallVec::new(),
            },
            collider,
            collision_group: CollisionGroups::new(ATTACK_GROUP, Group::all()),
        }
    }
}

#[derive(Component)]
pub struct AttackHitbox {
    pub damage: f32,
    pub knockback: Vec2,
    pub owner: Option<CharacterKind>,
    pub hit_list: SmallVec<[Entity; 4]>,
}

fn hit_detection(
    mut events: MessageReader<CollisionEvent>,
    mut attack_que: Query<&mut AttackHitbox>,
    character_que: Query<&Character>,
) {
    for &event in events.read() {
        let CollisionEvent::Started(e1, e2, _) = event else {
            continue;
        };
        let Some(attack_entity) = get_contained_entity(e1, e2, &attack_que) else {
            continue;
        };
        let Some(character_entity) = get_contained_entity(e1, e2, &character_que) else {
            continue;
        };
        let Ok(mut attack) = attack_que.get_mut(attack_entity) else {
            continue;
        };

        let Ok(character) = character_que.get(character_entity) else {
            continue;
        };
        if let Some(owner) = attack.owner {
            if owner == character.kind {
                continue;
            }
        }
        attack.hit_list.push(character_entity);
    }
}
