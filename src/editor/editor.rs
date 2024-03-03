use core::f32;
use std::usize;

use bevy::prelude::*;
use crate::config::{
    StateGlobal, StateEditorLoaded, StateEditorView,
    TRANSLATION_EDITOR_TILE_SELECTOR_ORIGIN,
};
use crate::common::level::{BundleTile, BundleTileLevel, LEVEL_ORIGIN, GridPosition};
use crate::common::asset_loader::SceneAssets;

const TILE_SELECTOR_VIEW_TILE_COLUMN_NUMBER: usize = 4;
const TILE_SPACING: usize = 1;


// -- BUNDLE : TILES ---------------------------------------------------------

#[derive(Resource, Debug, Default)]
pub struct TilesSelectionGrid {
    pub tile_vector: Vec<Handle<Scene>>,
}

// marker component
#[derive(Component)]
pub struct MarkerTileSelector;

#[derive(Component)]
pub struct MarkerTileSample;

// tiles used to define BundleTileBuilder type.
#[derive(Bundle)]
pub struct BundleTileSelector{
    pub tile: BundleTile,
    pub tile_type: MarkerTileSelector,
}

// singleton of the tile display to edit level.
#[derive(Bundle)]
pub struct BundleTileSample{
    pub tile: BundleTile,
}

// used to detect mouse cursor position with level editor.
#[derive(Component)]
struct GroundPlane;


// -- COMPONENT : GUI --------------------------------------------------------

#[derive(Component)]
struct MarkerEditorGUI;

#[derive(Component)]
pub struct MarkerTextLoadingEditor;


// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditor;

impl Plugin for PluginEditor{
    fn build(&self, app: &mut App){
        app.insert_resource(TilesSelectionGrid::default());
        app.add_systems(
            Update, 
            (
                user_input_editor.run_if(in_state(StateGlobal::Editor)),
                edit_level.run_if(in_state(StateGlobal::Editor)),
                editor_loading_prepare.run_if(
                    in_state(StateGlobal::Editor).and_then(
                    in_state(StateEditorLoaded::NotLoaded))
                ),
            ),
        );
        app.add_systems(OnEnter(StateEditorLoaded::Loading), editor_loading_load);
        app.add_systems(
            OnEnter(StateGlobal::Editor),
                editor_setup.run_if(in_state(StateEditorLoaded::Loaded))
        );
        app.add_systems(OnEnter(StateEditorLoaded::Loaded), editor_setup);
        app.add_systems(OnExit(StateGlobal::Editor), editor_teardown);
    }
}


// -- SYSTEM -----------------------------------------------------------------

fn editor_loading_prepare(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut tiles_selection_grid: ResMut<TilesSelectionGrid>,
    mut state_editor_loaded: ResMut<NextState<StateEditorLoaded>>,
) {

    tiles_selection_grid.tile_vector.push(scene_assets.tile_floor.clone());
    tiles_selection_grid.tile_vector.push(scene_assets.tile_fire.clone());
    tiles_selection_grid.tile_vector.push(scene_assets.tile_water.clone());
    tiles_selection_grid.tile_vector.push(scene_assets.tile_armoire.clone());
    tiles_selection_grid.tile_vector.push(scene_assets.tile_exit.clone());
    tiles_selection_grid.tile_vector.push(scene_assets.tile_table_1.clone());
    tiles_selection_grid.tile_vector.push(scene_assets.tile_table_2.clone());
    tiles_selection_grid.tile_vector.push(scene_assets.tile_wall_corner.clone());
    tiles_selection_grid.tile_vector.push(scene_assets.tile_wall_angle.clone());
    tiles_selection_grid.tile_vector.push(scene_assets.tile_wall.clone());
    tiles_selection_grid.tile_vector.push(scene_assets.title_desk.clone());

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
    entity: Query<Entity, With <MarkerTextLoadingEditor>>
) {

    let tile_number = tiles_selection_grid.tile_vector.len();
    let column_number = TILE_SELECTOR_VIEW_TILE_COLUMN_NUMBER;
    let row_number = tile_number / column_number + 1;
    
    let spawn_start_position = TRANSLATION_EDITOR_TILE_SELECTOR_ORIGIN - Vec3::new(
        row_number as f32,
        0.0,
        column_number as f32
    );

    let mut current_row:usize = 0;
    let mut current_col:usize = 0;
    for tile in tiles_selection_grid.tile_vector.iter_mut() {
        let position = spawn_start_position + Vec3::new(
            (current_row * 2 + current_row * TILE_SPACING) as f32,
            0.0,
            (current_col * 2 + current_col * TILE_SPACING) as f32,
        );
        commands.spawn(BundleTile{
            model: SceneBundle {
                scene: tile.clone(),
                transform: Transform::from_translation(position),
                ..default()
            }, 
            grid_position: GridPosition{
                value: IVec2::new(0, 0)
            }
        });

        if current_col >= TILE_SELECTOR_VIEW_TILE_COLUMN_NUMBER - 1{
            current_col = 0;
            current_row += 1;
        } else {
            current_col += 1;
        }
    }

    commands.entity(entity.single()).despawn();
    state_editor_loaded.set(StateEditorLoaded::Loaded);
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
    entity: Query<Entity, With <MarkerEditorGUI>>
) {
    commands.entity(entity.single()).despawn();
}


fn edit_level(mut commands: Commands) {
}


fn user_input_editor(
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


fn spawn_selector_entity(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn(BundleTileSelector{
        tile: BundleTile{
            model: SceneBundle {
                scene: scene_assets.tile_floor.clone(),
                transform: Transform::from_translation(LEVEL_ORIGIN),
                ..default()
            }, 
            grid_position: GridPosition{
                value: IVec2::new(0, 0)
            }
        },
        tile_type:MarkerTileSelector
    });
}

fn spawn_level_tile_from_selector(
        mut commands: Commands,
        selector_query: Query<(&GridPosition, &Transform, &Handle<Scene>), With <MarkerTileSelector>>
) {
    let (grid_position, transform, scene_handle) = selector_query.single();
    let level_tile = BundleTileLevel{
        tile : BundleTile {
            model: SceneBundle {
                scene: scene_handle.clone(),
                transform: transform.clone(),
                ..default()
            },
            grid_position: grid_position.clone()
        }
    };
    commands.spawn(level_tile);
}

