use core::f32;
use std::{usize, cmp};

use bevy::prelude::*;
use crate::config::StateGlobal;
use crate::common::tiles::{ResCollectionTile, BundleTile, GridPosition};
use crate::editor::common::{
    EventEditorSubSystemLoaded,
    EventTileSelectedChanged,
    StateEditorLoaded,
    StateEditorView,
    TRANSLATION_EDITOR_TILE_SELECTOR_ORIGIN,
};
use crate::editor::cursor_to_world::CursorToGroundCoordonate;

const TILE_SELECTOR_VIEW_TILE_COLUMN_NUMBER: usize = 4;
const TILE_SPACING: f32 = 0.5;

// -- COMPONENTS -------------------------------------------------------------

// TODO: split this ressource, no need to query tile_vector every frame!
// need to split between parts that are modified together...
#[derive(Resource, Debug, Default)]
pub struct TilesSelectionGrid {
    pub current_idx: usize,
    pub col_number: usize,
    pub row_number: usize,
    pub col_number_on_last_row: usize,
    pub translation_first_tile: Vec3,
}

// FIXME : ALL OF THIS RESSOURCE USELESS?
#[derive(Resource, Debug, Default)]
pub struct LevelBuilderInfo {
    pub selected_idx: usize,  // USELESS FOR NOW ...
    pub grid_pos_x: usize,
    pub grid_pos_z: usize,
}

#[derive(Component)]
struct MarkerTileSelectorCube;

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditorSelectTile;

impl Plugin for PluginEditorSelectTile{
    fn build(&self, app: &mut App){
        app
            .insert_resource(TilesSelectionGrid::default())
            .insert_resource(LevelBuilderInfo::default())
            .add_systems(OnEnter(StateEditorLoaded::Loading), load)
            .add_systems(
                Update,
                (
                    // TODO: put all of this in a system set....
                    snap_selector_cube_to_cursor_coord.run_if(
                        in_state(StateGlobal::Editor).and_then(
                        in_state(StateEditorLoaded::Ready).and_then(
                        in_state(StateEditorView::TileSelector)))
                    ),
                    user_input.run_if(
                        in_state(StateGlobal::Editor).and_then(
                        in_state(StateEditorLoaded::Ready).and_then(
                        in_state(StateEditorView::TileSelector)))
                    ),
                )
            );
    }
}

// -- SYSTEM -----------------------------------------------------------------

pub fn load(
    mut commands: Commands,
    mut tiles_selection_grid: ResMut<TilesSelectionGrid>,
    mut cursor_to_plane_coord: ResMut<CursorToGroundCoordonate>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut r_collection_tile: ResMut<ResCollectionTile>,
    mut e_editor_subsystem_loaded: EventWriter<EventEditorSubSystemLoaded>,
) {

    // 1. Filling ressources.

    let tile_number = r_collection_tile.tiles.len();

    tiles_selection_grid.col_number = TILE_SELECTOR_VIEW_TILE_COLUMN_NUMBER;
    let remainer = tile_number % tiles_selection_grid.col_number;
    let divider = tile_number / tiles_selection_grid.col_number;
    if remainer == 0 {
        tiles_selection_grid.row_number = divider;
        tiles_selection_grid.col_number_on_last_row = tiles_selection_grid.col_number;
    } else {
        tiles_selection_grid.row_number = divider + 1;
        tiles_selection_grid.col_number_on_last_row = remainer;
    }

    let translation_first_tile = TRANSLATION_EDITOR_TILE_SELECTOR_ORIGIN - Vec3::new(
        tiles_selection_grid.row_number as f32,
        0.0,
        tiles_selection_grid.col_number as f32
    );

    cursor_to_plane_coord.global = translation_first_tile;
    cursor_to_plane_coord.local = Vec2::ZERO;


    // 2. Spawning every tiles on "tile selector view".

    let mut current_row:usize = 0;
    let mut current_col:usize = 0;
    for tile in r_collection_tile.tiles.iter_mut() {
        let position = translation_first_tile + Vec3::new(
            current_row as f32 * 2.0 + current_row as f32 * TILE_SPACING,
            0.0,
            current_col as f32 * 2.0 + current_col as f32 * TILE_SPACING,
        );
        commands.spawn(BundleTile{
            model: SceneBundle {
                scene: tile.tile_model.clone(),
                transform: Transform::from_translation(position),
                ..default()
            }, 
            tile_id: tile.tile_id.clone(),
            grid_position: GridPosition {
                x : 0,
                z : 0,
            }
        });

        if current_col >= TILE_SELECTOR_VIEW_TILE_COLUMN_NUMBER - 1{
            current_col = 0;
            current_row += 1;
        } else {
            current_col += 1;
        }
    }

    tiles_selection_grid.translation_first_tile = translation_first_tile;

    // 3. Spawning cube used to select tiles in selector view.

    let selector_cube_size = 2.0 + TILE_SPACING / 2.0;
    commands.spawn(
        (
            PbrBundle {
                mesh: meshes.add(Cuboid::from_size(Vec3::ONE * selector_cube_size)),
                material: materials.add(Color::rgba(0.0, 0.7, 0.0, 0.5)),
                transform: Transform::from_translation(
                        tiles_selection_grid.translation_first_tile + Vec3::new(0.0, selector_cube_size/2.0, 0.0)),
                ..default()
            },
            MarkerTileSelectorCube,
        )
    );
    e_editor_subsystem_loaded.send(EventEditorSubSystemLoaded);
}

fn dispose(
    mut commands: Commands,
) {
}

fn user_input(
    r_mouse_input: Res<ButtonInput<MouseButton>>,
    // r_builder_info: Res<LevelBuilderInfo>,
    r_tile_selection_grid: Res<TilesSelectionGrid>,
    mut e_tile_selected_changed: EventWriter<EventTileSelectedChanged>,
    mut s_next_editor_view: ResMut<NextState<StateEditorView>>,
    
) {
    if r_mouse_input.just_pressed(MouseButton::Left) {
        e_tile_selected_changed.send(
            EventTileSelectedChanged{
                tile_id: r_tile_selection_grid.current_idx
            }
        );
        s_next_editor_view.set(StateEditorView::Level);
    }
} 

fn snap_selector_cube_to_cursor_coord(
    r_cursor_to_ground_coord: Res<CursorToGroundCoordonate>,
    mut r_tile_selection_grid: ResMut<TilesSelectionGrid>,
    mut q_selector_cube: Query<&mut Transform, With <MarkerTileSelectorCube>>,
) {

    let local_position = r_cursor_to_ground_coord.global - 
        r_tile_selection_grid.translation_first_tile;

    let grid_size = 2.0 + TILE_SPACING / 2.0;

    let grid_pos_x = cmp::max(
        cmp::min(
            ((local_position.x + (grid_size / 2.0)) / grid_size) as usize,
            r_tile_selection_grid.row_number - 1
        ),
        0
    );

    let grid_pos_z: usize;

    if grid_pos_x == r_tile_selection_grid.row_number - 1 {
        grid_pos_z = cmp::max(
            cmp::min(
                ((local_position.z + (grid_size / 2.0)) / grid_size) as usize,
                r_tile_selection_grid.col_number_on_last_row - 1
            ),
            0
        );
    } else {
        grid_pos_z = cmp::max(
            cmp::min(
                (local_position.z / grid_size) as usize,
                r_tile_selection_grid.col_number - 1
            ),
            0
        );
    }

    r_tile_selection_grid.current_idx = r_tile_selection_grid.col_number * 
        grid_pos_x + grid_pos_z;

    let mut transform = q_selector_cube.single_mut();

    let grid_size = 2.0 + TILE_SPACING;

    *transform = Transform::from_translation(
        r_tile_selection_grid.translation_first_tile + Vec3::new(
            grid_size * grid_pos_x as f32,
            1.0,
            grid_size * grid_pos_z as f32,
        )
    );
}
