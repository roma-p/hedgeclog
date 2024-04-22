use bevy::prelude::*;

use crate::config::{StateGlobal, StateUserInputAllowed};
use crate::editor::common::{
    PluginEditorData,
    StateEditorMode,
};
use crate::editor::cursor_to_world::PluginCursorToWorld;
use crate::editor::ui::PluginEditorUI;
use crate::editor::mode_tile::mode_tile::PluginEditorModeTile;
use crate::editor::load_setup::PluginLoadSetup;
use crate::editor::move_camera::PluginEditorCameraMovement;

use crate::editor::common::SSetEditor;


// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditor;

impl Plugin for PluginEditor{
    fn build(&self, app: &mut App){
        app
            // PLUGINS -------------------------------------------------------
            .add_plugins(PluginEditorData)
            .add_plugins(PluginCursorToWorld)
            .add_plugins(PluginEditorUI)
            .add_plugins(PluginEditorModeTile)
            .add_plugins(PluginLoadSetup)
            .add_plugins(PluginEditorCameraMovement)
            // USER INPUT ----------------------------------------------------
            .add_systems(Update, user_input_editor_global.in_set(SSetEditor::UserInput))
            .configure_sets(Update, SSetEditor::UserInput .run_if(
                in_state(StateGlobal::EditorRunning).and_then(
                in_state(StateUserInputAllowed::Allowed)))
            );
    }
}

// -- SYSTEM -----------------------------------------------------------------

// -- User input --

fn user_input_editor_global(
    r_keyboard_input: Res<ButtonInput<KeyCode>>,
    mut s_state_global: ResMut<NextState<StateGlobal>>,
    mut snext_editor_mode: ResMut<NextState<StateEditorMode>>,

) {
    // QUITTING EDITOR
    if r_keyboard_input.just_pressed(KeyCode::KeyQ) {
        s_state_global.set(StateGlobal::Game); 
        return
    }
    // TILE MODE
    if r_keyboard_input.just_pressed(KeyCode::KeyT) {
        snext_editor_mode.set(StateEditorMode::tile); 
        return
    }
    // NORMAL MODE
    if r_keyboard_input.just_pressed(KeyCode::Escape) {
        snext_editor_mode.set(StateEditorMode::normal); 
        return
    }
} 
