mod config;
mod editor;
mod game;
mod level;
mod asset;

use bevy::prelude::*;
use config::PluginConfig;
use game::debug::PluginDebug;

use crate::asset::asset_loader::PluginAssetLoader;
use crate::level::level::PluginLevel;

use game::game::PluginGame;
use editor::editor::PluginEditor;

fn main() {
    App::new()
        // Bevy built-ins.
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 200.0,
        })
        // Custom plugins.
        .add_plugins(PluginConfig)

        .add_plugins(PluginAssetLoader)

        .add_plugins(PluginLevel)
        .add_plugins(PluginEditor)
        .add_plugins(PluginGame)
        .add_plugins(PluginDebug)
        .run();
}
