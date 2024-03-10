use std::f32::consts::PI;
use std::{usize, cmp};

use bevy::prelude::*;

use crate::config::StateGlobal;
use crate::common::tiles::{
    BundleTile, EnumeTileBehaviour, ResCollectionTile, MarkerTileOnLevel, 
    GridPosition
};
use crate::common::level::{LEVEL_ORIGIN, LevelGrid};
use crate::editor::common::{EventTileSelectedChanged, StateEditorLoaded};
use crate::editor::cursor_to_world::CursorToGroundCoordonate;


// -- COMPONENTS / RESSOURCES STATES -----------------------------------------

// stores data between editor launch.
#[derive(Resource, Debug, Default)]
pub struct BufferedData {
    pub selected_idx: usize,
}

#[derive(Component)]
pub struct MarkerTileCreator;

// TODO: SPLIT IN TWO.
#[derive(Resource, Debug, Default)]
struct LevelBuilderInfo {
    pub grid_pos_x: usize,
    pub grid_pos_z: usize,
    pub current_hover_tile: Option<Entity>
}

#[derive(Event)]
pub struct EventTileCreatorMoved;

#[derive(Event)]
pub struct EventTileCreated;

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditorAddRemoveTile;

impl Plugin for PluginEditorAddRemoveTile{
    fn build(&self, app: &mut App){
        app
            .add_event::<EventTileCreatorMoved>()
            .add_event::<EventTileCreated>()
            .insert_resource(LevelBuilderInfo::default())
            .insert_resource(BufferedData::default())
            .add_systems(
                OnEnter(StateGlobal::Editor),
                    setup.run_if(in_state(StateEditorLoaded::LoadedNotSetup))
            )
            .add_systems(OnEnter(StateEditorLoaded::LoadedNotSetup), setup)
            .add_systems(OnExit(StateGlobal::Editor), teardown)
            .add_systems(
                Update,
                (
                    user_input
                        .run_if(in_state(StateGlobal::Editor)
                        .and_then(in_state(StateEditorLoaded::LoadedNotSetup))),
                    update_tile_creator_type
                        .run_if(on_event::<EventTileSelectedChanged>()),
                    update_cursor_grid_position,
                    update_tile_creator_position
                        .run_if(on_event::<EventTileCreatorMoved>()),
                    create_tile
                        .run_if(on_event::<EventTileCreated>())
                )
            );
    }
}

fn setup(
    mut commands: Commands,
    r_collection_tile: Res<ResCollectionTile>,
    r_buffered_data: Res<BufferedData>,
    mut r_level_builder_info: ResMut<LevelBuilderInfo>,
    mut e_tile_creator_moved: EventWriter<EventTileCreatorMoved>,
) {
    let tile_data = &r_collection_tile.tiles[r_buffered_data.selected_idx];
    commands.spawn(
        (
            BundleTile{
                model: SceneBundle {
                    scene: tile_data.tile_model.clone(),
                    transform: Transform::from_translation(LEVEL_ORIGIN),
                    ..default()
                }, 
                tile_id: tile_data.tile_id.clone(),
                grid_position: GridPosition {
                    x : 0,
                    z : 0,
                }
            },
            MarkerTileCreator,
        )
    );
    r_level_builder_info.grid_pos_x = 0;
    r_level_builder_info.grid_pos_z = 0;
    e_tile_creator_moved.send(EventTileCreatorMoved);
}

fn teardown(
    mut commands: Commands,
    q_tile_creator: Query<Entity, With <MarkerTileCreator>>,
    mut r_level_builder_info: ResMut<LevelBuilderInfo>,
    mut q_tiles: Query<(Entity, &mut Visibility), With <MarkerTileOnLevel>>
) {
    commands.entity(q_tile_creator.single()).despawn_recursive();
    if !r_level_builder_info.current_hover_tile.is_some() {
        return
    }
    for (entity, mut visibility) in q_tiles.iter_mut() {
        if entity == r_level_builder_info.current_hover_tile.unwrap() {
            *visibility = Visibility::Visible;
        }
    }
    r_level_builder_info.current_hover_tile = None;
}

fn user_input(
    r_mouse_input: Res<ButtonInput<MouseButton>>,
    r_keyboard_input: Res<ButtonInput<KeyCode>>,
    mut e_tile_selected_changed: EventWriter<EventTileCreated>,
    mut q_tile_creator: Query<&mut Transform, With <MarkerTileCreator>>,
) {
    if r_mouse_input.just_pressed(MouseButton::Left) {
        e_tile_selected_changed.send(EventTileCreated);
    } else if r_keyboard_input.just_pressed(KeyCode::KeyR) {
        let mut transform = q_tile_creator.single_mut();
        transform.rotate_local_y(PI/2.0);
    }
} 

fn update_tile_creator_type(
    mut commands: Commands,
    q_tile_creator: Query<Entity, With <MarkerTileCreator>>,
    r_collection_tile: Res<ResCollectionTile>,
    mut r_buffered_data: ResMut<BufferedData>,
    mut e_tile_selected_changed: EventReader<EventTileSelectedChanged>,
){
    commands.entity(q_tile_creator.single()).despawn_recursive();

    for ev in e_tile_selected_changed.read() {
        r_buffered_data.selected_idx = ev.tile_id;
    }

    // Spawning creator tile (used to edit level).
    let tile_data = &r_collection_tile.tiles[r_buffered_data.selected_idx];
    commands.spawn(
        (
            BundleTile{
                model: SceneBundle {
                    scene: tile_data.tile_model.clone(),
                    transform: Transform::from_translation(LEVEL_ORIGIN),
                    ..default()
                }, 
                tile_id: tile_data.tile_id.clone(),
                grid_position: GridPosition {
                    x : 0,
                    z : 0,
                }
            },
            MarkerTileCreator,
        )
    );
}

fn update_cursor_grid_position(
    mut r_level_builder_info: ResMut<LevelBuilderInfo>,
    r_cursor_to_ground_coordonate: Res<CursorToGroundCoordonate>,
    mut e_tile_creator_moved: EventWriter<EventTileCreatorMoved>,
) {
    let previous_grid_pos_x = r_level_builder_info.grid_pos_x;
    let previous_grid_pos_z = r_level_builder_info.grid_pos_z;

    let local_position = r_cursor_to_ground_coordonate.global - LEVEL_ORIGIN;

    let grid_size = 2.0;
    const LEVEL_SIZE:usize = 8;

    let grid_pos_x = cmp::max(
        cmp::min(
            ((local_position.x + (grid_size / 2.0)) / grid_size) as usize,
            LEVEL_SIZE - 1
        ),
        0
    );

    let grid_pos_z = cmp::max(
        cmp::min(
            ((local_position.z + (grid_size / 2.0)) / grid_size) as usize,
            LEVEL_SIZE - 1
        ),
        0
    );

    r_level_builder_info.grid_pos_x = grid_pos_x;
    r_level_builder_info.grid_pos_z = grid_pos_z;

    if previous_grid_pos_x != grid_pos_x || previous_grid_pos_z != grid_pos_z {
        e_tile_creator_moved.send(EventTileCreatorMoved);
    }
}

fn update_tile_creator_position(
    mut r_level_builder_info: ResMut<LevelBuilderInfo>,
    r_grid : Res<LevelGrid>,
    mut q_tile_creator: Query<&mut Transform, With <MarkerTileCreator>>,
    mut q_tiles: Query<(Entity, &GridPosition, &mut Visibility), With <MarkerTileOnLevel>>
){
    let mut entity_new_value: Option<Entity> = None;

    let grid_pos_x = r_level_builder_info.grid_pos_x;
    let grid_pos_z = r_level_builder_info.grid_pos_z;
    let current_tile_behaviour = r_grid.level_grid[grid_pos_x][grid_pos_z];

    let shall_make_previous_tile_visible = r_level_builder_info.current_hover_tile.is_some();

    let shall_make_current_tile_hidden: bool;
    match current_tile_behaviour {
        EnumeTileBehaviour::Empty => {
            shall_make_current_tile_hidden = false;
        },
        _ => {
            shall_make_current_tile_hidden = true;
        }
    }

    let mut previous_tile_found = false;
    let mut current_tile_found = false;

    for (entity, grid_position, mut visibility) in q_tiles.iter_mut() {

        if 
                r_level_builder_info.current_hover_tile.is_some() &&
                entity == r_level_builder_info.current_hover_tile.unwrap()
        {
            *visibility = Visibility::Visible;
            previous_tile_found = true;
        }

        if grid_pos_x == grid_position.x && grid_pos_z == grid_position.z {
            if shall_make_current_tile_hidden {
                *visibility = Visibility::Hidden;
                current_tile_found = true;
                entity_new_value = Some(entity);
            }
        }

        if 
            (!shall_make_current_tile_hidden || current_tile_found) &&
            (!shall_make_previous_tile_visible || previous_tile_found) {
            break
        }
    }

    r_level_builder_info.current_hover_tile = entity_new_value;

    let grid_size = 2.0;

    let mut transform = q_tile_creator.single_mut();
    let current_rotation = transform.rotation; 
    *transform = Transform::from_translation(
        LEVEL_ORIGIN + Vec3::new(
            grid_size * grid_pos_x as f32,
            0.0,
            grid_size * grid_pos_z as f32,
        ))
        .with_rotation(current_rotation);
}


fn create_tile(
    mut commands: Commands,
    r_level_builder_info: Res<LevelBuilderInfo>,
    r_buffered_data: Res<BufferedData>,
    r_collection_tile: Res<ResCollectionTile>,
    q_tile_creator: Query<&Transform, With <MarkerTileCreator>>,
    mut r_grid : ResMut<LevelGrid>,
) {

    let current_hover_tile = r_level_builder_info.current_hover_tile;
    if current_hover_tile.is_some(){
        commands.entity(current_hover_tile.unwrap()).despawn();
    }

    let tile = &r_collection_tile.tiles[r_buffered_data.selected_idx];
    let transform = q_tile_creator.single();

    let grid_pos_x = r_level_builder_info.grid_pos_x;
    let grid_pos_z = r_level_builder_info.grid_pos_z;

    commands.spawn(
        (
            BundleTile{
                model: SceneBundle {
                    scene: tile.tile_model.clone(),
                    transform: transform.clone(),
                    ..default()
                }, 
                tile_id: tile.tile_id.clone(),
                grid_position: GridPosition {
                    x : grid_pos_x,
                    z : grid_pos_z,
                }
            },
            MarkerTileOnLevel,
        ),
    );
    r_grid.level_grid[grid_pos_x][grid_pos_z] = tile.tile_behaviour;
}
