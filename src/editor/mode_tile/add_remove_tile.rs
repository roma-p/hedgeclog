use std::f32::consts::PI;
use std::usize;

use bevy::prelude::*;

use crate::config::{StateGlobal, StateUserInputAllowed};
use crate::common::common::GridPosition;
use crate::common::tiles::{
    BundleTile,
    EnumeTileBehaviour,
    ResCollectionTile,
    MarkerTileOnLevel
};
use crate::common::level::{LEVEL_ORIGIN, LevelGrid};
use crate::editor::common::{
    EventEditorSubSystemSetup,
    EventTileSelectedChanged,
    EventCursorGridPositionChanged,
    SSetEditor,
    StateEditorLoaded,
    StateEditorMode
};
use crate::editor::cursor_to_world::CursorGridPosition;


// -- COMPONENTS / RESSOURCES STATES -----------------------------------------

// stores data between editor launch.
#[derive(Resource, Debug, Default)]
pub struct BufferedData {
    pub selected_idx: usize,
}

#[derive(Component)]
pub struct MarkerTileCreator;

#[derive(Resource, Debug, Default)]
pub struct TileBuilderInfo {
    pub current_hover_tile: Option<Entity>,
    pub current_hover_position: GridPosition
}

#[derive(Event)]
pub struct EventTileCreated;

#[derive(Event)]
pub struct EventTileRemoved;

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditorAddRemoveTile;

impl Plugin for PluginEditorAddRemoveTile{
    fn build(&self, app: &mut App){
        app
            .add_event::<EventTileCreated>()
            .add_event::<EventTileRemoved>()
            .insert_resource(TileBuilderInfo::default())
            .insert_resource(BufferedData::default())
            .add_systems(OnEnter(StateEditorLoaded::LoadedAndSetuping), setup)
            .add_systems(OnExit(StateGlobal::EditorRunning), teardown)
            .add_systems(
                Update,
                (
                    user_input
                        .in_set(SSetEditor::UserInput)
                        .run_if(in_state(StateEditorMode::Tile)),
                    update_tile_creator_type
                        .run_if(on_event::<EventTileSelectedChanged>()),
                    update_tile_creator_position
                        .run_if(on_event::<EventCursorGridPositionChanged>()
                        .and_then(in_state(StateEditorMode::Tile))),
                    create_tile
                        .run_if(on_event::<EventTileCreated>()),
                    remove_tile
                        .run_if(on_event::<EventTileRemoved>()),
                )
            );
    }
}

fn setup(
    mut commands: Commands,
    r_collection_tile: Res<ResCollectionTile>,
    r_buffered_data: Res<BufferedData>,
    // mut r_level_builder_info: ResMut<TileBuilderInfo>,  // FIXME: to del?
    mut e_editor_subsystem_setup: EventWriter<EventEditorSubSystemSetup>,
) {
    let tile_data = &r_collection_tile.tiles[r_buffered_data.selected_idx];
    commands.spawn(
        (
            BundleTile{
                model: SceneBundle {
                    scene: tile_data.tile_model.clone(),
                    transform: Transform::from_translation(LEVEL_ORIGIN),
                    visibility: Visibility::Hidden,
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
    e_editor_subsystem_setup.send(EventEditorSubSystemSetup);
}

fn teardown(
    mut commands: Commands,
    q_tile_creator: Query<Entity, With <MarkerTileCreator>>,
    mut r_level_builder_info: ResMut<TileBuilderInfo>,
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
    r_level_builder_info.current_hover_position.x = 0;
    r_level_builder_info.current_hover_position.z = 0;
}

fn user_input(
    r_mouse_input: Res<ButtonInput<MouseButton>>,
    r_keyboard_input: Res<ButtonInput<KeyCode>>,
    mut e_tile_created: EventWriter<EventTileCreated>,
    mut e_tile_removed: EventWriter<EventTileRemoved>,
    mut q_tile_creator: Query<&mut Transform, With <MarkerTileCreator>>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
) {
    if r_mouse_input.just_pressed(MouseButton::Left) {
        s_user_input_allowed.set(StateUserInputAllowed::NotAllowed);  // -> set to Allowed by add_remove_tile.create_tile
        e_tile_created.send(EventTileCreated);
    } else if r_mouse_input.just_pressed(MouseButton::Right) {
        s_user_input_allowed.set(StateUserInputAllowed::NotAllowed);  // -> set to Allowed by add_remove_tile.remove_tile
        e_tile_removed.send(EventTileRemoved);
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
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
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
    s_user_input_allowed.set(StateUserInputAllowed::Allowed);
}

fn update_tile_creator_position(
    mut r_level_builder_info: ResMut<TileBuilderInfo>,
    r_cursor_grid_position: Res<CursorGridPosition>,
    r_grid : Res<LevelGrid>,
    mut q_tile_creator: Query<&mut Transform, With <MarkerTileCreator>>,
    mut q_tiles: Query<(Entity, &GridPosition, &mut Visibility), With <MarkerTileOnLevel>>,
){
    let mut entity_new_value: Option<Entity> = None;

    let grid_pos_x = r_cursor_grid_position.grid_pos_x;
    let grid_pos_z = r_cursor_grid_position.grid_pos_z;
    let current_tile_behaviour = r_grid.level_grid[grid_pos_x][grid_pos_z];

    let shall_make_previous_tile_visible = r_level_builder_info.current_hover_tile.is_some();

    let shall_make_current_tile_hidden = match current_tile_behaviour {
        EnumeTileBehaviour::Empty => false,
        _ => true
    };

    let mut previous_tile_found = false;
    let mut current_tile_found = false;

    for (entity, grid_position, mut visibility) in q_tiles.iter_mut() {

        // making previous hover tile visible.
        if 
                r_level_builder_info.current_hover_tile.is_some() &&
                entity == r_level_builder_info.current_hover_tile.unwrap()
        {
            *visibility = Visibility::Visible;
            previous_tile_found = true;
        }

        // making new hover tile hidden and registering it to ressource.
        if grid_pos_x == grid_position.x && grid_pos_z == grid_position.z {
            if shall_make_current_tile_hidden {
                *visibility = Visibility::Hidden;
                current_tile_found = true;
                entity_new_value = Some(entity);
            }
        }
        
        // when all relevant tiles are found, breaking.
        if 
            (!shall_make_current_tile_hidden || current_tile_found) &&
            (!shall_make_previous_tile_visible || previous_tile_found) {
            break
        }
    }

    r_level_builder_info.current_hover_tile = entity_new_value;
    r_level_builder_info.current_hover_position = GridPosition{
           x: grid_pos_x,
           z: grid_pos_z,
    };

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
    r_level_builder_info: Res<TileBuilderInfo>,
    r_buffered_data: Res<BufferedData>,
    r_collection_tile: Res<ResCollectionTile>,
    q_tile_creator: Query<&Transform, With <MarkerTileCreator>>,
    mut r_grid : ResMut<LevelGrid>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
) {

    let current_hover_tile = r_level_builder_info.current_hover_tile;
    if current_hover_tile.is_some(){
        // FIXME: crash here: verify that entity still exists...
        commands.entity(current_hover_tile.unwrap()).despawn();
    }

    let tile = &r_collection_tile.tiles[r_buffered_data.selected_idx];
    let transform = q_tile_creator.single();

    let grid_pos_x = r_level_builder_info.current_hover_position.x;
    let grid_pos_z = r_level_builder_info.current_hover_position.z;

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
    s_user_input_allowed.set(StateUserInputAllowed::Allowed);
}

// TODO:  When deleting tiles, also delete hedgehog.

fn remove_tile(
    mut commands: Commands,
    r_level_builder_info: Res<TileBuilderInfo>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
    mut r_grid : ResMut<LevelGrid>,
) {

    let grid_pos_x = r_level_builder_info.current_hover_position.x;
    let grid_pos_z = r_level_builder_info.current_hover_position.z;

    let current_hover_tile = r_level_builder_info.current_hover_tile;
    if current_hover_tile.is_some(){
        // FIXME: crash here: verify that entity still exists...
        commands.entity(current_hover_tile.unwrap()).despawn();
    }
    r_grid.level_grid[grid_pos_x][grid_pos_z] = EnumeTileBehaviour::Empty;
    s_user_input_allowed.set(StateUserInputAllowed::Allowed);
}
