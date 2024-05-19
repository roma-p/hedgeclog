mod config;
mod common;
mod editor;
mod game;

use bevy::prelude::*;

use config::PluginConfig;

use common::hedgehog::PluginHedghog;
use common::level::PluginLevel;
use common::camera::PluginCamera;
use game::debug::PluginDebug;
use common::asset_loader::PluginAssetLoader;
use common::tiles::PluginTiles;

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
        .add_plugins(PluginHedghog)
        .add_plugins(PluginLevel)
        .add_plugins(PluginCamera)
        .add_plugins(PluginEditor)
        .add_plugins(PluginGame)
        .add_plugins(PluginDebug)
        .add_plugins(PluginTiles)
        .run();
}
