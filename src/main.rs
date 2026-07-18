#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{prelude::*, window::WindowResolution};
use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;
mod character;
mod collision_groups;
mod config;
mod end_app;
mod enemy;
mod game_state;
mod game_system_set;
mod ground_state;
mod loading;
mod map;
mod menu;
mod player;
mod room_transition;
mod rooms;
mod select_data;
mod settings;
mod test_scene;
mod title;
mod utils;

pub use collision_groups::*;
pub use game_state::GameState;
pub use loading::JpFont;
pub use utils::*;

fn main() {
    let settings = settings::get_settings();
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: settings.window.title.clone(),
            resolution: WindowResolution::new(settings.window.width, settings.window.height),
            mode: if settings.window.fullscreen {
                bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
            } else {
                bevy::window::WindowMode::Windowed
            },
            present_mode: if settings.window.vsync {
                bevy::window::PresentMode::AutoVsync
            } else {
                bevy::window::PresentMode::AutoNoVsync
            },
            ..default()
        }),
        ..default()
    }));
    bevy::asset::embedded_asset!(app, "fonts/NotoSansJP-Bold.ttf");
    bevy::asset::embedded_asset!(app, "images/ground_texture.png");
    bevy::asset::embedded_asset!(app, "images/ground_upper_texture.png");
    bevy::asset::embedded_asset!(app, "images/one_way_ground.png");

    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(TimestepMode::Interpolated {
            dt: 1.0 / 120.0,
            time_scale: 1.0,
            substeps: 1,
        })
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(HanabiPlugin)
        .init_state::<GameState>()
        .insert_resource(settings)
        .add_plugins(config::ConfigPlugin)
        .add_plugins(loading::LoadingPlugin)
        .add_plugins(ground_state::GroundStatePlugin)
        .add_plugins(game_system_set::GameSystemSetPlugin)
        .add_plugins(utils::UtilsPlugin)
        .add_plugins(character::CharacterPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(title::TitlePlugin)
        .add_plugins(select_data::SelectDataPlugin)
        .add_plugins(end_app::EndAppPlugin)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(test_scene::TestScenePlugin)
        .add_plugins(room_transition::RoomTransitionPlugin)
        .add_plugins(rooms::RoomsPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .run();
}
