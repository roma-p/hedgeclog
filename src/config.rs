use bevy::prelude::*;

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

pub struct PluginConfig;

impl Plugin for PluginConfig{
    fn build(&self, app: &mut App){
        app.init_state::<StateGlobal>();
        app.init_state::<StateEditorLoaded>();
    }
}
