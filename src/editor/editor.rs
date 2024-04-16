use std::usize;

use bevy::prelude::*;

use crate::config::{StateGlobal, StateUserInputAllowed};
use crate::editor::common::{
    PluginEditorData,
    EventEditorSubSystemLoaded,
    EventEditorSubSystemSetup,
    StateEditorLoaded,
    StateEditorView,
    StateEditorMode,
};
use crate::editor::cursor_to_world::PluginCursorToWorld;
use crate::editor::ui::PluginEditorUI;
use crate::editor::mode_tile::mode_tile::PluginEditorModeTile;

use super::common::SSetEditor;

const SUBSYSTEM_TO_LOAD_NUMBER: usize = 2;
const SUBSYSTEM_TO_SETUP_NUMBER: usize = 2;

// -- COMPONENTS -------------------------------------------------------------

#[derive(Component)]
pub struct MarkerTextLoadingEditor;


#[derive(Resource, Default)]
struct ResourceSubSystemStatus {
    number_loaded: usize,
    number_setup: usize,
}

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditor;

impl Plugin for PluginEditor{
    fn build(&self, app: &mut App){
        app
            // INIT DATA -----------------------------------------------------
            .insert_resource(ResourceSubSystemStatus::default())
            // PLUGINS -------------------------------------------------------
            .add_plugins(PluginEditorData)
            .add_plugins(PluginCursorToWorld)
            .add_plugins(PluginEditorUI)
            .add_plugins(PluginEditorModeTile)
            // LOADING / DISPOSE ---------------------------------------------
            .add_systems(Update, load_prepare.run_if(
                in_state(StateGlobal::EditorRequested).and_then(
                in_state(StateEditorLoaded::NotLoaded)))
            )
            .add_systems(Update, load_do
                .run_if(on_event::<EventEditorSubSystemLoaded>())
            )
            // SETUP / TEARDOWN ----------------------------------------------
            .add_systems(
                OnEnter(StateGlobal::EditorRequested),
                    setup_prepare.run_if(in_state(StateEditorLoaded::LoadedNotSetup))
            )
            .add_systems(
                OnEnter(StateEditorLoaded::JustLoadedNeedSetup),
                setup_prepare
            )
            .add_systems(Update, setup_do
                .run_if(on_event::<EventEditorSubSystemSetup>())
            )
            .add_systems(OnExit(StateGlobal::EditorRunning), editor_teardown)
            // USER INPUT ----------------------------------------------------
            .add_systems(Update, user_input_editor_global.in_set(SSetEditor::UserInput))
            .configure_sets(Update, SSetEditor::UserInput .run_if(
                in_state(StateGlobal::EditorRunning).and_then(
                in_state(StateUserInputAllowed::Allowed)))
            );
    }
}

// -- SYSTEM -----------------------------------------------------------------

// -- loading --

fn load_prepare(
    mut commands: Commands,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
) {
    commands.spawn(
        (
            TextBundle::from_section(
                "Loading editor...",
                TextStyle {
                    font_size: 25.0,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            }),
            MarkerTextLoadingEditor,
        )
    );
    s_editor_loaded.set(StateEditorLoaded::Loading);
    s_user_input_allowed.set(StateUserInputAllowed::NotAllowed);
}

fn load_do(
    mut commands: Commands,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
    q_text_loading_editor: Query<Entity, With <MarkerTextLoadingEditor>>,
    mut r_sub_system_status: ResMut<ResourceSubSystemStatus>,
    mut e_editor_subsystem_loaded: EventReader<EventEditorSubSystemLoaded>,
) {
    for _ in e_editor_subsystem_loaded.read() {
        r_sub_system_status.number_loaded += 1;
    }
    if r_sub_system_status.number_loaded != SUBSYSTEM_TO_LOAD_NUMBER {
        return
    }
    commands.entity(q_text_loading_editor.single()).despawn();
    s_editor_loaded.set(StateEditorLoaded::JustLoadedNeedSetup);
}

fn setup_prepare(
    mut commands: Commands,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
    mut snext_editor_mode: ResMut<NextState<StateEditorMode>>,
) {
    commands.spawn(
        (
            TextBundle::from_section(
                "Loading editor...",
                TextStyle {
                    font_size: 25.0,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            }),
            MarkerTextLoadingEditor,
        )
    );
    s_editor_loaded.set(StateEditorLoaded::LoadedAndSetuping);
    s_user_input_allowed.set(StateUserInputAllowed::NotAllowed);
    snext_editor_mode.set(StateEditorMode::normal);
}

fn setup_do(
    mut commands: Commands,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
    mut s_global: ResMut<NextState<StateGlobal>>,
    q_text_loading_editor: Query<Entity, With <MarkerTextLoadingEditor>>,
    mut r_sub_system_status: ResMut<ResourceSubSystemStatus>,
    mut e_editor_subsystem_setup: EventReader<EventEditorSubSystemSetup>,
) {

    for _ in e_editor_subsystem_setup.read() {
        r_sub_system_status.number_setup += 1;
    }
    if r_sub_system_status.number_setup != SUBSYSTEM_TO_SETUP_NUMBER {
        return
    }

    commands.entity(q_text_loading_editor.single()).despawn();
    s_editor_loaded.set(StateEditorLoaded::Ready);
    s_user_input_allowed.set(StateUserInputAllowed::Allowed);
    s_global.set(StateGlobal::EditorRunning);
}

fn editor_teardown(
    mut r_sub_system_status: ResMut<ResourceSubSystemStatus>,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
    mut snext_editor_mode: ResMut<NextState<StateEditorMode>>,
) {
    r_sub_system_status.number_setup = 0;
    s_editor_loaded.set(StateEditorLoaded::LoadedNotSetup);
    snext_editor_mode.set(StateEditorMode::NoSet);
}

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
