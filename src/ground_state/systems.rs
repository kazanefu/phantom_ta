use crate::config::PlayerConfig;

use super::*;

pub fn update_ground_state(
    rapier_context: ReadRapierContext,
    ground_query: Query<(), With<Ground>>,
    mut query: Query<(Entity, &mut GroundState), With<Collider>>,
    config: Res<PlayerConfig>,
) {
    let Ok(ctx) = rapier_context.single() else {
        return;
    };

    for (entity, mut state) in &mut query {
        state.contact_flag = ContactFlag::empty();
        state.contacts.clear();
        state.platform = None;

        for pair in ctx.contact_pairs_with(entity) {
            if !pair.has_any_active_contact() {
                continue;
            }

            let Some(collider1) = pair.collider1() else {
                continue;
            };

            let Some(collider2) = pair.collider2() else {
                continue;
            };

            let other = if collider1 == entity {
                collider2
            } else if collider2 == entity {
                collider1
            } else {
                continue;
            };

            if !ground_query.contains(other) {
                continue;
            }

            for manifold in pair.manifolds() {
                let mut normal = manifold.normal();

                if collider1 == entity {
                    normal = -normal;
                }

                let normal = Vec2::new(normal.x, normal.y);

                state.contacts.push(ContactInfo {
                    entity: other,
                    normal,
                });

                if normal.y > config.control.ground_threshold {
                    state.contact_flag |= ContactFlag::OnGround;

                    state.platform.get_or_insert(other);
                }

                if normal.y < -config.control.ground_threshold {
                    state.contact_flag |= ContactFlag::OnCeiling;
                }

                if normal.x.abs() > config.control.ground_threshold {
                    state.contact_flag |= ContactFlag::OnWall;
                }
            }
        }
    }
}
