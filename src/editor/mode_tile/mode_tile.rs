use bevy::prelude::*;
use crate::editor::common::{
    StateEditorMode,
    StateEditorView, 
    EventCursorGridPositionChanged
};
use crate::common::level::LevelGrid;
use crate::editor::mode_tile::common::{ModeTileLocalBuffer, PluginEditorModeTileCommon};
use crate::editor::mode_tile::select_tile::PluginEditorSelectTile;
use crate::editor::mode_tile::add_remove_tile::{
    PluginEditorAddRemoveTile,
    MarkerTileCreator,
};

use crate::editor::common::SSetEditor;
use crate::common::tiles::MarkerTileOnLevel;



// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditorModeTile;

// TODO: set input mode tile... ???

impl Plugin for PluginEditorModeTile{
    fn build(&self, app: &mut App){
        app
            .insert_resource(ModeTileLocalBuffer::default())
            .add_plugins(PluginEditorSelectTile)
            .add_plugins(PluginEditorModeTileCommon)
            .add_plugins(PluginEditorAddRemoveTile)
            .add_systems(OnEnter(StateEditorMode::Tile), enter_mode_tile)
            .add_systems(OnExit(StateEditorMode::Tile), exit_mode_tile)
            .add_systems(Update, user_input_editor_mode_tile
                .in_set(SSetEditor::UserInput)
                .run_if(in_state(StateEditorMode::Tile))
            );
    }
}

// -- SYSTEM -----------------------------------------------------------------

fn enter_mode_tile(
    mut q_tile_creator: Query< &mut Visibility, With <MarkerTileCreator>>,
    mut e_tile_creator_moved: EventWriter<EventCursorGridPositionChanged>,
) {
    let mut visibility = q_tile_creator.single_mut();
    *visibility = Visibility::Visible;
    e_tile_creator_moved.send(EventCursorGridPositionChanged);
}

fn exit_mode_tile(
    mut q_tile_creator: Query< &mut Visibility, (With <MarkerTileCreator>, Without<MarkerTileOnLevel>)>,
    r_local_buffer: Res<ModeTileLocalBuffer>,
    r_grid : Res<LevelGrid>,
    mut q_tiles: Query<(Entity, &mut Visibility), (With <MarkerTileOnLevel>, Without<MarkerTileCreator>)>
) {

    if let Ok(mut visibility) = q_tile_creator.get_single_mut() {
        *visibility = Visibility::Hidden;
    }

    if !r_local_buffer.hover_tile_grid_position.is_some() {
        return
    }

    let grid_pos = r_local_buffer.hover_tile_grid_position.unwrap();
    let hover_entity = r_grid.level_grid[grid_pos.x][grid_pos.z].tile_entity.unwrap();
    for (entity, mut visibility) in q_tiles.iter_mut() {
        if entity == hover_entity {
            *visibility = Visibility::Visible;
        }
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
