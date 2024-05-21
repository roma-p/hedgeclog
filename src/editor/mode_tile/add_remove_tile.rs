use std::f32::consts::PI;

use bevy::prelude::*;

use crate::config::{StateGlobal, StateUserInputAllowed};
use crate::common::common::GridPosition;
use crate::common::tiles::{
    BundleTile,
    EnumeTileBehaviour,
    ResCollectionTile,
    MarkerTileOnLevel, 
    TILE_SIZE
};
use crate::common::level::{
    EventTileCreationAsked, EventTileRemovalAsked, LevelGrid, LEVEL_ORIGIN
};
use crate::editor::common::{
    EventEditorSubSystemSetup,
    EventTileSelectedChanged,
    EventCursorGridPositionChanged,
    SSetEditor,
    StateEditorLoaded,
    StateEditorMode
};
use crate::editor::cursor_to_world::CursorGridPosition;
use crate::editor::mode_tile::mode_tile::ModeTileLocalBuffer;


// -- COMPONENTS / RESSOURCES STATES -----------------------------------------
#[derive(Component)]
pub struct MarkerTileCreator;

#[derive(Event)]
struct EventEditorTileCreationAsked;

#[derive(Event)]
struct EventEditorTileRemovalAsked;

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditorAddRemoveTile;

impl Plugin for PluginEditorAddRemoveTile{
    fn build(&self, app: &mut App){
        app
            .add_event::<EventEditorTileCreationAsked>()
            .add_event::<EventEditorTileRemovalAsked>()
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
                        .run_if(on_event::<EventEditorTileCreationAsked>()),
                    remove_tile
                        .run_if(on_event::<EventEditorTileRemovalAsked>()),
                )
            );
    }
}

fn setup(
    mut commands: Commands,
    r_collection_tile: Res<ResCollectionTile>,
    r_local_buffer: Res<ModeTileLocalBuffer>,
    mut e_editor_subsystem_setup: EventWriter<EventEditorSubSystemSetup>,
) {
    let tile_data = &r_collection_tile.tiles[r_local_buffer.selected_idx];
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
    mut r_local_buffer: ResMut<ModeTileLocalBuffer>,
    r_grid : Res<LevelGrid>,
    mut q_tiles: Query<(Entity, &mut Visibility), With <MarkerTileOnLevel>>
) {
    commands.entity(q_tile_creator.single()).despawn_recursive();

    if r_local_buffer.hover_tile_grid_position.is_some() {

        let grid = r_local_buffer.hover_tile_grid_position.unwrap();
        let entity = r_grid.level_grid[grid.x][grid.z].tile_entity;
        if entity.is_some() {
            for (i_entity, mut visibility) in q_tiles.iter_mut() {
                if i_entity == entity.unwrap() {
                    *visibility = Visibility::Visible;
                }
            }
        }
    }
    r_local_buffer.hover_tile_grid_position = None;
}

fn user_input(
    r_mouse_input: Res<ButtonInput<MouseButton>>,
    r_keyboard_input: Res<ButtonInput<KeyCode>>,
    mut e_tile_created: EventWriter<EventEditorTileCreationAsked>,
    mut e_tile_removed: EventWriter<EventEditorTileRemovalAsked>,
    mut q_tile_creator: Query<&mut Transform, With <MarkerTileCreator>>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
) {
    if r_mouse_input.just_pressed(MouseButton::Left) {
        s_user_input_allowed.set(StateUserInputAllowed::NotAllowed);  // -> set to Allowed by add_remove_tile.create_tile
        e_tile_created.send(EventEditorTileCreationAsked);
    } else if r_mouse_input.just_pressed(MouseButton::Right) {
        s_user_input_allowed.set(StateUserInputAllowed::NotAllowed);  // -> set to Allowed by add_remove_tile.remove_tile
        e_tile_removed.send(EventEditorTileRemovalAsked);
    } else if r_keyboard_input.just_pressed(KeyCode::KeyR) {
        let mut transform = q_tile_creator.single_mut();
        transform.rotate_local_y(PI/2.0);
    }
} 

fn update_tile_creator_type(
    mut commands: Commands,
    q_tile_creator: Query<Entity, With <MarkerTileCreator>>,
    r_collection_tile: Res<ResCollectionTile>,
    mut r_local_buffer: ResMut<ModeTileLocalBuffer>,
    mut e_tile_selected_changed: EventReader<EventTileSelectedChanged>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
){

    commands.entity(q_tile_creator.single()).despawn_recursive();
    for ev in e_tile_selected_changed.read() {
        r_local_buffer.selected_idx = ev.tile_id;
    }

    // Spawning creator tile (used to edit level).
    let tile_data = &r_collection_tile.tiles[r_local_buffer.selected_idx];
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
    r_cursor_grid_position: Res<CursorGridPosition>,
    r_grid : Res<LevelGrid>,
    mut r_local_buffer: ResMut<ModeTileLocalBuffer>,
    mut q_tile_creator: Query<&mut Transform, With <MarkerTileCreator>>,
    mut q_tiles: Query<(Entity, &GridPosition, &mut Visibility), With <MarkerTileOnLevel>>,
){
    let grid_pos_x = r_cursor_grid_position.grid_pos_x;
    let grid_pos_z = r_cursor_grid_position.grid_pos_z;
    let current_tile_behaviour = r_grid.level_grid[grid_pos_x][grid_pos_z].tile_behaviour;

    let mut previous_tile_entity: Option<Entity> = None;
    let mut shall_make_previous_tile_visible = false;
    
    if r_local_buffer.hover_tile_grid_position.is_some(){
        let grid_pos = r_local_buffer.hover_tile_grid_position.unwrap();
        let opt = r_grid.level_grid[grid_pos.x][grid_pos.z].tile_entity;
        if opt.is_some() {
            shall_make_previous_tile_visible = true;
            previous_tile_entity = opt;
        }
    }

    if shall_make_previous_tile_visible {
        let grid_pos = r_local_buffer.hover_tile_grid_position.unwrap();
        previous_tile_entity = r_grid.level_grid[grid_pos.x][grid_pos.z].tile_entity;
    }

    let shall_make_current_tile_hidden = match current_tile_behaviour {
        EnumeTileBehaviour::Empty => false,
        _ => true
    };

    let mut previous_tile_found = false;
    let mut current_tile_found = false;

    for (entity, grid_position, mut visibility) in q_tiles.iter_mut() {

        // making previous hover tile visible.
        if 
                shall_make_previous_tile_visible &&
                entity == previous_tile_entity.unwrap()
        {
            *visibility = Visibility::Visible;
            previous_tile_found = true;
        }

        // making new hover tile hidden and registering it to ressource.
        if grid_pos_x == grid_position.x && grid_pos_z == grid_position.z {
            if shall_make_current_tile_hidden {
                *visibility = Visibility::Hidden;
                current_tile_found = true;
            }
        }
        
        // when all relevant tiles are found, breaking.
        if 
            (!shall_make_current_tile_hidden || current_tile_found) &&
            (!shall_make_previous_tile_visible || previous_tile_found) {
            break
        }
    }

    r_local_buffer.hover_tile_grid_position = Some(GridPosition{
           x: grid_pos_x,
           z: grid_pos_z,
    });

    let mut transform = q_tile_creator.single_mut();
    let current_rotation = transform.rotation; 
    *transform = Transform::from_translation(
        LEVEL_ORIGIN + Vec3::new(
            TILE_SIZE * grid_pos_x as f32,
            0.0,
            TILE_SIZE * grid_pos_z as f32,
        ))
        .with_rotation(current_rotation);
}


fn create_tile(
    r_local_buffer: Res<ModeTileLocalBuffer>,
    q_tile_creator: Query<&Transform, With <MarkerTileCreator>>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
    mut e_event_tile_creation_asked: EventWriter<EventTileCreationAsked>,
    r_cursor_grid_position: Res<CursorGridPosition>,
    
) {
    e_event_tile_creation_asked.send(
        EventTileCreationAsked{
            tile_idx: r_local_buffer.selected_idx,
            tile_transform: q_tile_creator.single().clone(),
            grid_position: GridPosition {
                x: r_cursor_grid_position.grid_pos_x,
                z: r_cursor_grid_position.grid_pos_z,
            }
        }
    );
    // TODO: centralize this logic to! I don't know how...
    s_user_input_allowed.set(StateUserInputAllowed::Allowed);
}

fn remove_tile(
    r_cursor_grid_position: Res<CursorGridPosition>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
    mut e_event_tile_removal_asked: EventWriter<EventTileRemovalAsked>,
) {
    e_event_tile_removal_asked.send(EventTileRemovalAsked{
        grid_position: GridPosition {
            x: r_cursor_grid_position.grid_pos_x,
            z: r_cursor_grid_position.grid_pos_z,
        }
    });
    s_user_input_allowed.set(StateUserInputAllowed::Allowed);
}
