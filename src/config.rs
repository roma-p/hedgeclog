use std::usize;

use bevy::prelude::*;

pub const TRANSLATION_LEVEL_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const TRANSLATION_EDITOR_TILE_SELECTOR_ORIGIN: Vec3 = Vec3::new(1000.0, 0.0, 1000.0);
pub const TRANSLATION_DEFAULT_CAMERA_SHIFT: Vec3 = Vec3::new(9.0, 9.0, 9.0);

pub const LEVEL_DEFAULT_SIZE: usize = 10;

// Used to transition from game to editor at any time.
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum StateGlobal {
    #[default]
    Game,
    Editor,
}

// Used to check if editor is loaded or not.
// Will only be loaded when editor first call 
// (to not built it by default in release game mode)
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum StateEditorLoaded {
    #[default]
    NotLoaded,
    Loading,
    Loaded,
}

// Used to check if level is loaded or not.
// Use when loading new level.
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum StateLevelLoaded {
    #[default]
    NotLoaded,
    Loading,
    Loaded,
}

// Used to check if level is loaded or not.
// Use when loading new level.
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum StateEditorView {
    #[default]
    Game,
    TileSelector,
}


pub struct PluginConfig;

impl Plugin for PluginConfig{
    fn build(&self, app: &mut App){
        app.init_state::<StateGlobal>();
        app.init_state::<StateLevelLoaded>();
        app.init_state::<StateEditorLoaded>();
        app.init_state::<StateEditorView>();
    }
}
