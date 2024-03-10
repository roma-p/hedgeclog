use std::usize;

use bevy::prelude::*;

pub const TRANSLATION_EDITOR_TILE_SELECTOR_ORIGIN: Vec3 = Vec3::new(
    1000.0, 0.0, 1000.0
);

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum StateEditorLoaded {
    #[default]
    NotLoaded,
    Loading,
    LoadedNotSetup,
    LoadedAndSetup,
}

// Used to check if level is loaded or not.
// Use when loading new level.
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum StateEditorView {
    #[default]
    Level,
    TileSelector,
}

#[derive(Event)]
pub struct EventEditorSubSystemLoaded;

#[derive(Event)]
pub struct EventTileSelectedChanged{
    pub tile_id: usize
}

pub struct PluginEditorData;

impl Plugin for PluginEditorData{
    fn build(&self, app: &mut App){
        app
            // INIT DATA -----------------------------------------------------
            .init_state::<StateEditorLoaded>()
            .init_state::<StateEditorView>()
            .add_event::<EventEditorSubSystemLoaded>()
            .add_event::<EventTileSelectedChanged>();
    }
}
