use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{ground_state::Ground, rooms::items::ItemKind, rooms::spawn::SpawnItemMsg};

use crate::{GameState, loading::*};

pub struct GroundSpawnPlugin;

impl Plugin for GroundSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            load_ground_texture.run_if(in_state(GameState::Loading).and(
                |tasklist: Res<LoadTaskState>| !tasklist.is_task_done(LoadTaskKind::GroundTexture),
            )),
        )
        .add_systems(Update, spawn_ground);
    }
}

#[derive(Resource)]
struct GroundTexture {
    base_texture: Handle<Image>,
    upper_texture: Handle<Image>,
}

fn load_ground_texture(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut tasklist: ResMut<LoadTaskState>,
) {
    let texture = asset_server.load("embedded://phantom_ta/images/ground_texture.png");
    let upper_texture = asset_server.load("embedded://phantom_ta/images/ground_upper_texture.png");
    commands.insert_resource(GroundTexture {
        base_texture: texture,
        upper_texture,
    });
    tasklist.set_task_done(LoadTaskKind::GroundTexture);
    println!("done ground texture");
}

fn spawn_ground(
    mut commands: Commands,
    mut msg: MessageReader<SpawnItemMsg>,
    ground_texture: Option<Res<GroundTexture>>,
) {
    let Some(ground_texture) = ground_texture else {
        return;
    };

    for msg in msg.read() {
        if msg.0.kind != ItemKind::Ground {
            continue;
        }
        let transform = msg.0.transform;
        let _physics = commands
            .spawn((
                Ground,
                RigidBody::Fixed,
                Transform {
                    translation: transform.translation,
                    rotation: transform.rotation,
                    scale: Vec3::splat(1.0),
                },
                Collider::cuboid(transform.scale.x / 2.0, transform.scale.y / 2.0),
            ))
            .id();
        let tile_size = Vec2::new(50.0, 50.0);
        for pos in crate::utils::tile::rect_tile_pos_iter(transform, tile_size) {
            let _tile = commands
                .spawn((
                    Sprite {
                        image: ground_texture.base_texture.clone(),
                        custom_size: Some(tile_size),
                        ..default()
                    },
                    Transform::from_translation(pos).with_rotation(transform.rotation),
                ))
                .id();
        }
        for pos in crate::utils::tile::upper_tile_pos_iter(transform, tile_size) {
            let _tile = commands
                .spawn((
                    Sprite {
                        image: ground_texture.upper_texture.clone(),
                        custom_size: Some(tile_size),
                        ..default()
                    },
                    Transform::from_translation(pos).with_rotation(transform.rotation),
                ))
                .id();
        }
    }
}
