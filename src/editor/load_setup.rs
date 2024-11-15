use bevy::prelude::*;
use std::usize;

use crate::config::{StateGlobal, StateUserInputAllowed};
use crate::editor::common::{
    EventEditorSubSystemLoaded, EventEditorSubSystemSetup, StateEditorLoaded, StateEditorMode,
};

// TODO: explain this and list all systems loading in //

const SUBSYSTEM_TO_LOAD_NUMBER: usize = 3;
// - editor.cursor_to_world.load
// - editor.select_tile.load
// - editor.move_camera.load

const SUBSYSTEM_TO_SETUP_NUMBER: usize = 4;
// - editor.add_remove_tile.setup
// - editor.add_remove_hedgehog.setup
// - editor.ui.setup
// - editor.cursor_to_world.setup

// -- COMPONENTS -------------------------------------------------------------

#[derive(Component)]
pub struct MarkerTextLoadingEditor;

#[derive(Resource, Default)]
struct ResourceSubSystemStatus {
    number_loaded: usize,
    number_setup: usize,
}

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginLoadSetup;

impl Plugin for PluginLoadSetup {
    fn build(&self, app: &mut App) {
        app
            // INIT DATA -----------------------------------------------------
            .insert_resource(ResourceSubSystemStatus::default())
            // LOADING / DISPOSE ---------------------------------------------
            .add_systems(
                Update,
                s_load_prepare.run_if(
                    in_state(StateGlobal::EditorRequested)
                        .and_then(in_state(StateEditorLoaded::NotLoaded)),
                ),
            )
            .add_systems(
                Update,
                s_load_do.run_if(on_event::<EventEditorSubSystemLoaded>()),
            )
            // SETUP / TEARDOWN ----------------------------------------------
            .add_systems(
                OnEnter(StateGlobal::EditorRequested),
                s_setup_prepare.run_if(in_state(StateEditorLoaded::LoadedNotSetup)),
            )
            .add_systems(
                OnEnter(StateEditorLoaded::JustLoadedNeedSetup),
                s_setup_prepare,
            )
            .add_systems(
                Update,
                s_setup_do.run_if(on_event::<EventEditorSubSystemSetup>()),
            )
            .add_systems(OnExit(StateGlobal::EditorRunning), s_editor_teardown);
    }
}

// -- SYSTEM -----------------------------------------------------------------

// -- loading --

fn s_load_prepare(
    mut commands: Commands,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
) {
    commands.spawn((
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
    ));
    s_editor_loaded.set(StateEditorLoaded::Loading);
    s_user_input_allowed.set(StateUserInputAllowed::NotAllowed);
}

fn s_load_do(
    mut commands: Commands,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
    q_text_loading_editor: Query<Entity, With<MarkerTextLoadingEditor>>,
    mut r_sub_system_status: ResMut<ResourceSubSystemStatus>,
    mut e_editor_subsystem_loaded: EventReader<EventEditorSubSystemLoaded>,
) {
    for _ in e_editor_subsystem_loaded.read() {
        r_sub_system_status.number_loaded += 1;
    }
    if r_sub_system_status.number_loaded != SUBSYSTEM_TO_LOAD_NUMBER {
        return;
    }
    commands.entity(q_text_loading_editor.single()).despawn();
    s_editor_loaded.set(StateEditorLoaded::JustLoadedNeedSetup);
}

fn s_setup_prepare(
    mut commands: Commands,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
    mut snext_editor_mode: ResMut<NextState<StateEditorMode>>,
) {
    commands.spawn((
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
    ));
    s_editor_loaded.set(StateEditorLoaded::LoadedAndSetuping);
    s_user_input_allowed.set(StateUserInputAllowed::NotAllowed);
    snext_editor_mode.set(StateEditorMode::Normal);
}

fn s_setup_do(
    mut commands: Commands,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
    mut s_global: ResMut<NextState<StateGlobal>>,
    q_text_loading_editor: Query<Entity, With<MarkerTextLoadingEditor>>,
    mut r_sub_system_status: ResMut<ResourceSubSystemStatus>,
    mut e_editor_subsystem_setup: EventReader<EventEditorSubSystemSetup>,
) {
    for _ in e_editor_subsystem_setup.read() {
        r_sub_system_status.number_setup += 1;
    }
    if r_sub_system_status.number_setup != SUBSYSTEM_TO_SETUP_NUMBER {
        return;
    }

    commands.entity(q_text_loading_editor.single()).despawn();
    s_editor_loaded.set(StateEditorLoaded::Ready);
    s_user_input_allowed.set(StateUserInputAllowed::Allowed);
    s_global.set(StateGlobal::EditorRunning);
}

fn s_editor_teardown(
    mut r_sub_system_status: ResMut<ResourceSubSystemStatus>,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
    mut snext_editor_mode: ResMut<NextState<StateEditorMode>>,
) {
    r_sub_system_status.number_setup = 0;
    s_editor_loaded.set(StateEditorLoaded::LoadedNotSetup);
    snext_editor_mode.set(StateEditorMode::NoSet);
}
