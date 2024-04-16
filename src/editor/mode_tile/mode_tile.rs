use bevy::prelude::*;
use bevy::render::view::visibility;
use crate::config::{StateGlobal};
use crate::editor::common::{
    StateEditorMode, StateEditorView
};
use crate::editor::mode_tile::select_tile::PluginEditorSelectTile;
use crate::editor::mode_tile::add_remove_tile::{
    PluginEditorAddRemoveTile,
    MarkerTileCreator,
    EventTileCreatorMoved
};

use crate::editor::common::SSetEditor;


// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditorModeTile;

// TODO: set input mode tile... ???

impl Plugin for PluginEditorModeTile{
    fn build(&self, app: &mut App){
        app
            .add_plugins(PluginEditorSelectTile)
            .add_plugins(PluginEditorAddRemoveTile)
            .add_systems(OnEnter(StateEditorMode::tile), enter_mode_tile)
            .add_systems(OnExit(StateEditorMode::tile), exit_mode_tile)
            .add_systems(Update, user_input_editor_mode_tile
                .in_set(SSetEditor::UserInput)
                .run_if(in_state(StateEditorMode::tile))
            );
    }
}

// -- SYSTEM -----------------------------------------------------------------

fn enter_mode_tile(
    mut q_tile_creator: Query< &mut Visibility, With <MarkerTileCreator>>,
    mut e_tile_creator_moved: EventWriter<EventTileCreatorMoved>,
) {
    let mut visibility = q_tile_creator.single_mut();
    *visibility = Visibility::Visible;
    e_tile_creator_moved.send(EventTileCreatorMoved);
}

fn exit_mode_tile(
    mut q_tile_creator: Query< &mut Visibility, With <MarkerTileCreator>>
) {

    if let Ok(mut visibility) = q_tile_creator.get_single_mut() {
        *visibility = Visibility::Hidden;
    }
    // TODO force camera view to main view... :
    // currently done on camera... to be changed...
}

fn user_input_editor_mode_tile(
    r_keyboard_input: Res<ButtonInput<KeyCode>>,
    s_editor_view: Res<State<StateEditorView>>,
    mut s_next_editor_view: ResMut<NextState<StateEditorView>>,

) {

    // ENTERRING / LEAVING TILE SELECTION SCREEN.
    if r_keyboard_input.just_pressed(KeyCode::Space) {
        use StateEditorView::*;
        let next = match **s_editor_view {
            Level => TileSelector,
            TileSelector => Level,
        };
        s_next_editor_view.set(next);
    }
} 
