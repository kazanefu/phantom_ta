#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{prelude::*, window::WindowResolution};
use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;
mod config;
mod game_state;
mod settings;

pub use game_state::GameState;

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
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(TimestepMode::Interpolated {
            dt: 1.0 / 120.0,
            time_scale: 1.0,
            substeps: 1,
        })
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(HanabiPlugin)
        .init_state::<GameState>()
        .insert_resource(settings)
        .add_plugins(config::ConfigPlugin)
        .run();
}
