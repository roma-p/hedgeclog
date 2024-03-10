use std::usize;

use bevy::prelude::*;

use crate::config::StateGlobal;
use crate::editor::common::{
    PluginEditorData,
    EventEditorSubSystemLoaded,
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

// -- COMPONENTS -------------------------------------------------------------

#[derive(Component)]
pub struct MarkerTextLoadingEditor;

#[derive(Component)]
struct MarkerEditorGUI;

#[derive(Resource, Default)]
struct ResourceLoadedSubSystem {
    number: usize,
}

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditor;

impl Plugin for PluginEditor{
    fn build(&self, app: &mut App){
        app
            // INIT DATA -----------------------------------------------------
            .insert_resource(ResourceLoadedSubSystem::default())
            // PLUGINS -------------------------------------------------------
            .add_plugins(PluginEditorData)
            .add_plugins(PluginCursorToWorld)
            .add_plugins(PluginEditorAddRemoveTile)
            .add_plugins(PluginEditorSelectTile)
            // LOADING / DISPOSE ---------------------------------------------
            .add_systems(Update, editor_loading_prepare.run_if(
                in_state(StateGlobal::Editor).and_then(
                in_state(StateEditorLoaded::NotLoaded)))
            )
            .add_systems(Update, editor_loading_load
                .run_if(on_event::<EventEditorSubSystemLoaded>())
            )
            // SETUP / TEARDOWN ----------------------------------------------
            .add_systems(
                OnEnter(StateGlobal::Editor),
                    editor_setup.run_if(in_state(StateEditorLoaded::LoadedNotSetup))
            )
            .add_systems(OnEnter(StateEditorLoaded::LoadedNotSetup), editor_setup)
            .add_systems(OnExit(StateGlobal::Editor), editor_teardown)
            // USER INPUT ----------------------------------------------------
            .add_systems(Update, user_input_editor_global
                .run_if(in_state(StateGlobal::Editor))
            );
    }
}

// -- SYSTEM -----------------------------------------------------------------

// -- loading --

fn editor_loading_prepare(
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

fn editor_loading_load(
    mut commands: Commands,
    mut s_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
    q_text_loading_editor: Query<Entity, With <MarkerTextLoadingEditor>>,
    mut r_loaded_sub_system: ResMut<ResourceLoadedSubSystem>,
    mut e_editor_subsystem_loaded: EventReader<EventEditorSubSystemLoaded>,
) {
    for _ in e_editor_subsystem_loaded.read() {
        r_loaded_sub_system.number += 1;
    }
    if r_loaded_sub_system.number != SUBSYSTEM_TO_LOAD_NUMBER {
        return
    }
    commands.entity(q_text_loading_editor.single()).despawn();
    s_editor_loaded.set(StateEditorLoaded::LoadedNotSetup);
}

fn editor_setup(
    mut commands: Commands,
) {
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
}

fn editor_teardown(
    mut commands: Commands,
    q_editor_text: Query<Entity, With <MarkerEditorGUI>>,
) {
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
