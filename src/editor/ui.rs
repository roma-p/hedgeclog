use bevy::prelude::*;

use crate::config::{StateGlobal, StateUserInputAllowed};

use crate::editor::common::{
    StateEditorLoaded,
    StateEditorMode,
    MarkerEditorGUI,
    EventEditorSubSystemSetup,
};

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditorUI;

impl Plugin for PluginEditorUI{
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(StateEditorLoaded::LoadedAndSetuping), setup)
            .add_systems(OnExit(StateGlobal::EditorRunning), teardown)
            .add_systems(OnEnter(StateGlobal::EditorRunning), text_by_mode_normal)  // FIXME: hack! shall be removed.
            .add_systems(
                OnEnter(StateEditorMode::normal),
                text_by_mode_normal.run_if(in_state(StateGlobal::EditorRunning))
            )
            .add_systems(
                OnEnter(StateEditorMode::tile),
                text_by_mode_tile.run_if(in_state(StateGlobal::EditorRunning))
            );

    }
}

// -- TEXT BY MODE CONSTANTS -------------------------------------------------

const TEXT_MODE_HEADER_1: &str = r#"Level editor

current mode: "#;

const TEXT_MODE_HEADER_2: &str = r#"

available commands: 

q:   quit editor

ESC: mode normal
t:   mode "tile"
h:   mode "hedgehog"
x:   mode "try"

"#;

const TEXT_MODE_NORMAL: &str = r#"
"#;

const TEXT_MODE_TILE: &str = r#" Tile command

SPACE:       choose tile
r:           rotate tile
Left click:  add tile.
Right click: remove tile.
"#;

// -- SYTEMS -----------------------------------------------------------------

fn setup(
    mut commands: Commands,
    mut e_editor_subsystem_setup: EventWriter<EventEditorSubSystemSetup>,
) {
    commands.spawn(
        (
            TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 20.0,
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
    e_editor_subsystem_setup.send(EventEditorSubSystemSetup);
}

fn teardown(
    mut commands: Commands,
    q_text_loading_editor: Query<Entity, With <MarkerEditorGUI>>,
) {
    commands.entity(q_text_loading_editor.single()).despawn();
}


fn text_by_mode_normal(
    mut q_editor_text: Query<&mut Text, With <MarkerEditorGUI>>,
    s_editor_mode: Res<State<StateEditorMode>>,
) {
    let mut text = q_editor_text.single_mut();
    let text = &mut text.sections[0].value;
    text.clear();
    text.push_str(TEXT_MODE_HEADER_1);
    text.push_str(&format!("{:?}", s_editor_mode.get()));
    text.push_str(TEXT_MODE_HEADER_2);
    text.push_str(TEXT_MODE_NORMAL);
}

fn text_by_mode_tile(
    mut q_editor_text: Query<&mut Text, With <MarkerEditorGUI>>,
    s_editor_mode: Res<State<StateEditorMode>>,
) {
    let mut text = q_editor_text.single_mut();
    let text = &mut text.sections[0].value;
    text.clear();
    text.push_str(TEXT_MODE_HEADER_1);
    text.push_str(&format!("{:?}", s_editor_mode.get()));
    text.push_str(TEXT_MODE_HEADER_2);
    text.push_str(TEXT_MODE_TILE);
}

