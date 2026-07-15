use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    GameState, ONE_WAY_PLATFORM_GROUP,
    ground_state::Ground,
    loading::*,
    rooms::{RoomEntity, items::ItemKind, spawn::SpawnItemMsg},
};

pub struct OneWaySpawnPlugin;

impl Plugin for OneWaySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            load_one_way_texture.run_if(in_state(GameState::Loading).and(
                |tasklist: Res<LoadTaskState>| !tasklist.is_task_done(LoadTaskKind::OneWayTexture),
            )),
        )
        .add_systems(Update, spawn_one_way);
    }
}

#[derive(Resource)]
struct OneWayTexture {
    upper_texture: Handle<Image>,
}

fn load_one_way_texture(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut tasklist: ResMut<LoadTaskState>,
) {
    let upper_texture = asset_server.load("embedded://phantom_ta/images/one_way_ground.png");
    commands.insert_resource(OneWayTexture { upper_texture });
    tasklist.set_task_done(LoadTaskKind::OneWayTexture);
    println!("done one way texture");
}

fn spawn_one_way(
    mut commands: Commands,
    mut msg: MessageReader<SpawnItemMsg>,
    texture: Option<Res<OneWayTexture>>,
) {
    let Some(texture) = texture else {
        return;
    };

    for msg in msg.read() {
        if msg.0.kind != ItemKind::OneWay {
            continue;
        }

        let transform = msg.0.transform;
        commands.spawn((
            Ground,
            RigidBody::Fixed,
            CollisionGroups::new(ONE_WAY_PLATFORM_GROUP, Group::all()),
            Transform {
                translation: transform.translation,
                rotation: transform.rotation,
                scale: Vec3::splat(1.0),
            },
            Collider::cuboid(transform.scale.x / 2.0, transform.scale.y / 2.0),
            RoomEntity,
        ));

        let tile_size = Vec2::new(50.0, 50.0);
        for pos in crate::utils::tile::upper_tile_pos_iter(transform, tile_size) {
            commands.spawn((
                Sprite {
                    image: texture.upper_texture.clone(),
                    custom_size: Some(tile_size),
                    ..default()
                },
                Transform::from_translation(pos).with_rotation(transform.rotation),
                RoomEntity,
            ));
        }
    }
}
