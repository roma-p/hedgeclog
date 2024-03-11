use std::usize;

use bevy::prelude::*;

use crate::config::StateGlobal;
use crate::editor::common::{
    PluginEditorData,
    EventEditorSubSystemLoaded,
    EventEditorSubSystemSetup,
    StateEditorLoaded,
    StateEditorView
};
use crate::editor::cursor_to_world::{
    PluginCursorToWorld,
};
use crate::editor::select_tile::{
    PluginEditorSelectTile,
};
use crate::editor::add_remove_tile::PluginEditorAddRemoveTile;

const SUBSYSTEM_TO_LOAD_NUMBER: usize = 2;
const SUBSYSTEM_TO_SETUP_NUMBER: usize = 1;

// -- COMPONENTS -------------------------------------------------------------

#[derive(Component)]
pub struct MarkerTextLoadingEditor;

#[derive(Component)]
struct MarkerEditorGUI;

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
            .add_plugins(PluginEditorAddRemoveTile)
            .add_plugins(PluginEditorSelectTile)
            // LOADING / DISPOSE ---------------------------------------------
            .add_systems(Update, load_prepare.run_if(
                in_state(StateGlobal::Editor).and_then(
                in_state(StateEditorLoaded::NotLoaded)))
            )
            .add_systems(Update, load_do
                .run_if(on_event::<EventEditorSubSystemLoaded>())
            )
            // SETUP / TEARDOWN ----------------------------------------------
            .add_systems(
                OnEnter(StateGlobal::Editor),
                    setup_prepare.run_if(in_state(StateEditorLoaded::LoadedNotSetup))
            )
            .add_systems(
                OnEnter(StateEditorLoaded::JustLoadedNeedSetup),
                setup_prepare
            )
            .add_systems(Update, setup_do
                .run_if(on_event::<EventEditorSubSystemSetup>())
            )
            .add_systems(OnExit(StateGlobal::Editor), editor_teardown)
            // USER INPUT ----------------------------------------------------
            .add_systems(Update, user_input_editor_global .run_if(
                in_state(StateGlobal::Editor).and_then(
                in_state(StateEditorLoaded::Ready)))
            );
    }
}

// -- SYSTEM -----------------------------------------------------------------

// -- loading --

fn load_prepare(
    mut commands: Commands,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
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
}

fn setup_do(
    mut commands: Commands,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
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
    commands.spawn(
        (
            TextBundle::from_section(
                "Level editor.",
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
            MarkerEditorGUI,
        )
    );
    s_editor_loaded.set(StateEditorLoaded::Ready);
}

fn editor_teardown(
    mut commands: Commands,
    q_editor_text: Query<Entity, With <MarkerEditorGUI>>,
    mut r_sub_system_status: ResMut<ResourceSubSystemStatus>,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
) {
    r_sub_system_status.number_setup = 0;
    s_editor_loaded.set(StateEditorLoaded::LoadedNotSetup);
    commands.entity(q_editor_text.single()).despawn();
}

// -- User input --

fn user_input_editor_global(
    r_keyboard_input: Res<ButtonInput<KeyCode>>,
    mut s_state_global: ResMut<NextState<StateGlobal>>,
    s_editor_view: Res<State<StateEditorView>>,
    mut s_next_editor_view: ResMut<NextState<StateEditorView>>,

) {
    // QUITTING EDITOR
    if r_keyboard_input.just_pressed(KeyCode::Escape) {
        s_state_global.set(StateGlobal::Game); 
    }

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
