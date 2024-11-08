use bevy::prelude::*;
use crate::editor::mode_hedgehog::add_remove_hedgehog::{
    PluginAddRemoveHedgehog,
    MarkerHedgehogCreator,
};
use crate::editor::common::{
    StateEditorMode,
    EventCursorGridPositionChanged
};

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditorModeHedgeclog;

// TODO: set input mode tile... ???

impl Plugin for PluginEditorModeHedgeclog{
    fn build(&self, app: &mut App){
        app
            .add_plugins(PluginAddRemoveHedgehog)
            .add_systems(OnEnter(StateEditorMode::Hedgehog), s_enter_mode_hedgehog)
            .add_systems(OnExit(StateEditorMode::Hedgehog), s_exit_mode_hedgehog);
    }
}

// -- SYSTEM -----------------------------------------------------------------

fn s_enter_mode_hedgehog(
    mut q_h_creator: Query< &mut Visibility, With <MarkerHedgehogCreator>>,
    mut e_tile_creator_moved: EventWriter<EventCursorGridPositionChanged>,
) {
    let mut visibility = q_h_creator.single_mut();
    *visibility = Visibility::Visible;
    e_tile_creator_moved.send(EventCursorGridPositionChanged);
}

fn s_exit_mode_hedgehog(
    mut q_h_creator: Query< &mut Visibility, With <MarkerHedgehogCreator>>
) {

    if let Ok(mut visibility) = q_h_creator.get_single_mut() {
        *visibility = Visibility::Hidden;
    }
    // TODO force camera view to main view... :
    // currently done on camera... to be changed...
}
