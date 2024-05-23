use bevy::prelude::*;


// Used to transition from game to editor at any time.
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum StateGlobal {
    #[default]
    Game,
    EditorRequested,
    EditorRunning,
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum StateUserInputAllowed {
    #[default]
    NotAllowed,
    Allowed
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

pub struct PluginConfig;

impl Plugin for PluginConfig{
    fn build(&self, app: &mut App){
        app
            .init_state::<StateGlobal>()
            .init_state::<StateLevelLoaded>()
            .init_state::<StateUserInputAllowed>();
    }
}


// NEXT -> user_input handled as set for a start.
// Then... how to simplify load / setup ...
// Then rework loading of level?
