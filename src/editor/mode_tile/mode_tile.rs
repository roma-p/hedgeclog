use bevy::prelude::*;
use crate::config::{StateGlobal};
use crate::editor::common::{
    StateEditorMode, StateEditorView
};
use crate::editor::mode_tile::select_tile::PluginEditorSelectTile;
use crate::editor::mode_tile::add_remove_tile::PluginEditorAddRemoveTile;

use crate::editor::common::SSetEditor;


// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditorModeTile;

// TODO: set input mode tile... ???

impl Plugin for PluginEditorModeTile{
    fn build(&self, app: &mut App){
        app
            .add_plugins(PluginEditorSelectTile)
            .add_plugins(PluginEditorAddRemoveTile)
            .add_systems(Update, user_input_editor_mode_tile
                .in_set(SSetEditor::UserInput)
                .run_if(in_state(StateEditorMode::tile))
            );
    }
}

// -- SYSTEM -----------------------------------------------------------------

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
