use core::f32;
use std::{usize, cmp};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::config::{
    StateGlobal, StateEditorLoaded, StateEditorView,
    TRANSLATION_EDITOR_TILE_SELECTOR_ORIGIN,
};
use crate::common::level::{GridPosition, LEVEL_ORIGIN};
use crate::common::tiles::{BundleTile, ResCollectionTile};
use crate::common::camera::MarkerCamera;

const TILE_SELECTOR_VIEW_TILE_COLUMN_NUMBER: usize = 4;
const TILE_SPACING: f32 = 0.5;


// -- BUNDLE : TILES ---------------------------------------------------------

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

#[derive(Resource, Debug, Default)]
pub struct LevelBuilderInfo {
    pub selected_idx: usize,
    pub grid_pos_x: usize,
    pub grid_pos_z: usize,
}

#[derive(Resource, Default)]
struct CursorToGroundCoordonate {
    global: Vec3,
    local: Vec2,
}

// used to detect mouse cursor position with level editor.
#[derive(Component)]
struct MarkerGroundPlane;


// marker component
#[derive(Component)]
pub struct MarkerTileCreator;

#[derive(Component)]
pub struct MarkerTileSample;

#[derive(Component)]
struct MarkerTileSelectorCube;

// -- COMPONENT : GUI --------------------------------------------------------

#[derive(Component)]
struct MarkerEditorGUI;

#[derive(Component)]
pub struct MarkerTextLoadingEditor;


// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditor;

impl Plugin for PluginEditor{
    fn build(&self, app: &mut App){
        app.insert_resource(TilesSelectionGrid::default())
            .insert_resource(CursorToGroundCoordonate::default())
            .add_systems(OnEnter(StateEditorLoaded::Loading), editor_loading_load)
            .add_systems(
                OnEnter(StateGlobal::Editor),
                    editor_setup.run_if(in_state(StateEditorLoaded::Loaded))
            )
            .add_systems(
                OnEnter(StateEditorView::Game),
                update_tile_creator.run_if(in_state(StateEditorLoaded::Loaded)))
            .add_systems(OnEnter(StateEditorLoaded::Loaded), editor_setup)
            .add_systems(OnExit(StateGlobal::Editor), editor_teardown)
            .add_systems(
                Update, 
                (
                    user_input_editor_global.run_if(in_state(StateGlobal::Editor)),
                    editor_loading_prepare.run_if(
                        in_state(StateGlobal::Editor).and_then(
                        in_state(StateEditorLoaded::NotLoaded))
                    ),
                    compute_cursor_to_ground_coordonate.run_if(
                        in_state(StateGlobal::Editor).and_then(
                        in_state(StateEditorLoaded::Loaded))
                    ),
                    snap_selector_cube_to_cursor_coord.run_if(
                        in_state(StateGlobal::Editor).and_then(
                        in_state(StateEditorView::TileSelector))
                    ),
                    user_input_editor_tile_selection.run_if(
                        in_state(StateEditorView::TileSelector)
                    ),
                    snap_tile_creator_to_cursor_coord.run_if(
                        in_state(StateGlobal::Editor).and_then(
                        in_state(StateEditorLoaded::Loaded).and_then(
                        in_state(StateEditorView::Game)))
                    ),
                ),
            );
    }
}
// -- SYSTEM -----------------------------------------------------------------

// -- loading ----------------------------------------------------------------

fn editor_loading_prepare(
    mut commands: Commands,
    mut state_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
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
    state_editor_loaded.set(StateEditorLoaded::Loading);
}

fn editor_loading_load(
    mut commands: Commands,
    mut tiles_selection_grid: ResMut<TilesSelectionGrid>,
    mut state_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
    mut cursor_to_plane_coord: ResMut<CursorToGroundCoordonate>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut r_collection_tile: ResMut<ResCollectionTile>,
    entity: Query<Entity, With <MarkerTextLoadingEditor>>
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

    // 4. Spawning ground plane used to do "Cursor to 3D coord"

    commands.spawn((
        MarkerGroundPlane,
        PbrBundle {
            transform: Transform::default(),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));

    // Finally: dispose loading state.

    commands.entity(entity.single()).despawn();
    state_editor_loaded.set(StateEditorLoaded::Loaded);
}


// -- setup / teardown -------------------------------------------------------

fn editor_setup(
    mut commands: Commands,
    r_collection_tile: Res<ResCollectionTile>,
    r_builder_info: Res<LevelBuilderInfo>,
) {

    let tile_idx = r_builder_info.selected_idx;
    commands.spawn(
        (
            BundleTile{
                model: SceneBundle {
                    scene: r_collection_tile.tiles[tile_idx].tile_model.clone(),
                    transform: Transform::from_translation(LEVEL_ORIGIN),
                    ..default()
                }, 
                tile_id: r_collection_tile.tiles[tile_idx].tile_id.clone(),
            },
            MarkerTileCreator,
        )
    );

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
    q_tile_creator: Query<Entity, With <MarkerTileCreator>>
) {
    commands.entity(q_editor_text.single()).despawn();
    commands.entity(q_tile_creator.single()).despawn_recursive();
}


// User input systems --------------------------------------------------------

fn user_input_editor_global(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state_global: ResMut<NextState<StateGlobal>>,
    state_editor_view: Res<State<StateEditorView>>,
    mut state_next_editor_view: ResMut<NextState<StateEditorView>>,

) {
    // QUITTING EDITOR
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state_global.set(StateGlobal::Game); 
    }

    // ENTERRING / LEAVING TILE SELECTION SCREEN.
    if keyboard_input.just_pressed(KeyCode::Space) {
        use StateEditorView::*;
        let next = match **state_editor_view {
            Game => TileSelector,
            TileSelector => Game,
        };
        state_next_editor_view.set(next);
    }
} 

fn user_input_editor_tile_selection(
    buttons: Res<ButtonInput<MouseButton>>,
    r_tile_selection_grid: Res<TilesSelectionGrid>,
    mut r_builder_info: ResMut<LevelBuilderInfo>,
    mut state_next_editor_view: ResMut<NextState<StateEditorView>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        r_builder_info.selected_idx = r_tile_selection_grid.current_idx;
        state_next_editor_view.set(StateEditorView::Game);
    }
}

fn user_input_editor_game(
    buttons: Res<ButtonInput<MouseButton>>,
    mut r_tile_selection_grid: ResMut<TilesSelectionGrid>,
) {
    if buttons.just_pressed(MouseButton::Left) {
    }
}

// -- selector view ----------------------------------------------------------

fn update_tile_creator(
    mut commands: Commands,
    q_tile_creator: Query<Entity, With <MarkerTileCreator>>,
    r_collection_tile: Res<ResCollectionTile>,
    r_builder_info: Res<LevelBuilderInfo>,
){
    commands.entity(q_tile_creator.single()).despawn_recursive();

    // Spawning creator tile (used to edit level).
    let tile_idx = r_builder_info.selected_idx;
    commands.spawn(
        (
            BundleTile{
                model: SceneBundle {
                    scene: r_collection_tile.tiles[tile_idx].tile_model.clone(),
                    transform: Transform::from_translation(LEVEL_ORIGIN),
                    ..default()
                }, 
                tile_id: r_collection_tile.tiles[tile_idx].tile_id.clone(),
            },
            MarkerTileCreator,
        )
    );


}

fn snap_selector_cube_to_cursor_coord(
    r_cursor_to_ground_coord: Res<CursorToGroundCoordonate>,
    mut r_tile_selection_grid: ResMut<TilesSelectionGrid>,
    mut q_selector_cube: Query<&mut Transform, With <MarkerTileSelectorCube>>,
    ) {

    let local_position = r_cursor_to_ground_coord.global - r_tile_selection_grid.translation_first_tile;

    let grid_size = 2.0 + TILE_SPACING / 2.0;

    let grid_pos_x = cmp::max(
        cmp::min(
            (local_position.x / grid_size) as usize,
            r_tile_selection_grid.row_number - 1
        ),
        0
    );

    let grid_pos_z: usize;

    if grid_pos_x == r_tile_selection_grid.row_number - 1 {
        grid_pos_z = cmp::max(
            cmp::min(
                (local_position.z / grid_size) as usize,
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

    r_tile_selection_grid.current_idx = r_tile_selection_grid.col_number * grid_pos_x + grid_pos_z;

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


// -- game view --------------------------------------------------------------

fn snap_tile_creator_to_cursor_coord(
    r_cursor_to_ground_coord: Res<CursorToGroundCoordonate>,
    mut r_level_builder_info: ResMut<LevelBuilderInfo>,
    mut q_tile_creator: Query<&mut Transform, With <MarkerTileCreator>>,
    ) {

    let local_position = r_cursor_to_ground_coord.global - LEVEL_ORIGIN;

    let grid_size = 2.0;
    const LEVEL_SIZE:usize = 8;

    let grid_pos_x = cmp::max(
        cmp::min(
            (local_position.x / grid_size) as usize,
            LEVEL_SIZE - 1
        ),
        0
    );

    let grid_pos_z = cmp::max(
        cmp::min(
            (local_position.z / grid_size) as usize,
            LEVEL_SIZE - 1
        ),
        0
    );

    r_level_builder_info.grid_pos_x = grid_pos_x;
    r_level_builder_info.grid_pos_z = grid_pos_z;

    let mut transform = q_tile_creator.single_mut();
    *transform = Transform::from_translation(
        LEVEL_ORIGIN + Vec3::new(
            grid_size * grid_pos_x as f32,
            0.0,
            grid_size * grid_pos_z as f32,
        )
    );
}

// MISC ----------------------------------------------------------------------

fn compute_cursor_to_ground_coordonate(
    mut cursor_to_ground_coord: ResMut<CursorToGroundCoordonate>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MarkerCamera>>,
    q_plane: Query<&GlobalTransform, With<MarkerGroundPlane>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let ground_transform = q_plane.single();
    let window = q_window.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let plane_origin = ground_transform.translation();
    let plane_normal = ground_transform.up();

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // TODO: new this Plane at loading.
    let Some(distance) = ray.intersect_plane(plane_origin, Plane3d::new(plane_normal)) else {
        return;
    };

    let global_cursor = ray.get_point(distance);
    cursor_to_ground_coord.global = global_cursor;

    let inverse_transform_matrix = ground_transform.compute_matrix().inverse();
    let local_cursor = inverse_transform_matrix.transform_point3(global_cursor);
    cursor_to_ground_coord.local = local_cursor.xz();
}
