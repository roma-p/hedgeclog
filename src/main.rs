mod movement;
mod debug;
mod hedgehog;
mod level;
mod camera;
mod asset_loader;
mod level_editor;

use bevy::prelude::*;
use movement::PluginMovement;
use debug::PluginDebug;
use hedgehog::PluginHedghog;
use level::PluginLevel;
use camera::PluginCamera;
use asset_loader::PluginAssetLoader;
use level_editor::PluginLevelEditor;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum StateGlobal {
    #[default]
    Playing,
    Editing,
}

fn main() {
    App::new()
        // Bevy built-ins.
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })

        // Custom plugins.
        .add_state::<StateGlobal>()
        .add_plugins(PluginAssetLoader)
        .add_plugins(PluginHedghog)
        .add_plugins(PluginMovement)
        .add_plugins(PluginLevel)
        .add_plugins(PluginCamera)
        .add_plugins(PluginLevelEditor)
        // .add_plugins(PluginDebug)
        .run();
}
