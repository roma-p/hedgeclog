use bevy::prelude::*;

use crate::asset::asset_loader::PluginAssetLoader;
use crate::episode::episode::PluginEpsiode;
use crate::level::level::PluginLevel;
use crate::config::PluginConfig;
use crate::game::game::PluginGame;
use crate::game::debug::PluginDebug;
use crate::editor::editor::PluginEditor;
use crate::app::common::{
    PluginAppCommon,
    EPISODE_LOCATION_GAME,
    EPISODE_LOCATION_TESTBED,
    EPISODE_LOCATION_CONSTRUCT,
    ResEpisodeLocation,
};

pub fn add_plugins_default(app: &mut App){
    app
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
        .add_plugins(PluginEpsiode)
        .add_plugins(PluginLevel)
        .add_plugins(PluginGame)
        .add_plugins(PluginAppCommon);
}

pub fn add_plugins_dev(app: &mut App){
    app
        .add_plugins(PluginEditor)
        .add_plugins(PluginDebug);
}

// TODO: move to common?
pub enum AppType {
    Production,
    Construct,
    Testbed
}

pub fn set_episode_location(app: &mut App, app_type: AppType) {
    let episode_location = match app_type {
        AppType::Production => EPISODE_LOCATION_GAME,
        AppType::Construct => EPISODE_LOCATION_CONSTRUCT,
        AppType::Testbed => EPISODE_LOCATION_TESTBED,

    };
    app.insert_resource(
        ResEpisodeLocation{
            episode_location:episode_location.to_string()
        }
    );

}
