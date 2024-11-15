use bevy::prelude::*;

use crate::level::definition::level_definition::{
    GridPosition,
    ResCurrentLevelGrid,
    LevelGridHedgehog,
    LevelGridTile,
    ResCurrentLevel,
    LevelUid
};
use crate::level::definition::hedgehog::{
    EnumHedgehogOnGrid,
    BundleHedgehog,
    MarkerHedgehogOnLevel,
    HedgehogType,
};
use crate::asset::asset_loader::HedgehogAssets;
use crate::level::definition::tiles::{
    EnumeTileBehaviour,
    ResCollectionTile,
    BundleTile,
    MarkerTileOnLevel

};

#[derive(Event)]
pub struct EventTileCreationAsked{
    pub tile_idx: usize,
    pub tile_transform: Transform,
    pub grid_position: GridPosition,
}

#[derive(Event)]
pub struct EventTileRemovalAsked{
    pub grid_position: GridPosition,
}

#[derive(Event)]
pub struct EventLevelEdidted;

#[derive(Event)]
pub struct EventHedgehogCreationAsked{
    pub hedgehog_transform: Transform,
    pub grid_position: GridPosition,
}

#[derive(Event)]
pub struct EventHedgehogRemovalAsked{
    pub grid_position: GridPosition,
}

enum TileValidationPayload{
    AddedHedgehog,
    AddedTile,
    RemovedHedgehog,
    RemovedTile,
}

#[derive(Event)]
struct EventTileValidationAsked{
    pub grid_position: GridPosition,
    pub validation_payload: TileValidationPayload,
}

pub struct PluginEditLevel;

impl Plugin for PluginEditLevel{
    fn build(&self, app: &mut App){
        app
            .add_event::<EventTileCreationAsked>()
            .add_event::<EventTileRemovalAsked>()
            .add_event::<EventHedgehogCreationAsked>()
            .add_event::<EventHedgehogRemovalAsked>()
            .add_event::<EventTileValidationAsked>()
            .add_event::<EventLevelEdidted>()
            .add_systems(
                Update,
                (
                    s_create_tile.run_if(on_event::<EventTileCreationAsked>()),
                    s_remove_tile.run_if(on_event::<EventTileRemovalAsked>()),
                    s_create_hedgehog.run_if(on_event::<EventHedgehogCreationAsked>()),
                    s_remove_hedgehog.run_if(on_event::<EventHedgehogRemovalAsked>()),
                    s_validate_level_edition.run_if(on_event::<EventTileValidationAsked>())
                )
            );
    }
}

// -- FUNCTIONS --------------------------------------------------------------

fn remove_hedgehog(
    commands: &mut Commands,
    r_grid: &mut ResMut<ResCurrentLevelGrid>,
    x: usize, z: usize,
){
    if let Some(entity) = r_grid.hedgehog_grid[x][z].hedgehog_entity{
        commands.entity(entity).despawn();
        r_grid.hedgehog_grid[x][z] = LevelGridHedgehog{
            hedgehog_entity: None,
            hedgehog_behaviour: EnumHedgehogOnGrid::Empty,
            hedgehog_tile: None
        };
    }
}

fn remove_tile(
    commands: &mut Commands,
    r_grid: &mut ResMut<ResCurrentLevelGrid>,
    x: usize, z: usize,
){
    if let Some(tile_entity) = r_grid.level_grid[x][z].tile_entity{
        commands.entity(tile_entity).despawn();
        r_grid.level_grid[x][z] = LevelGridTile{
            tile_id: None,
            tile_entity: None,
            tile_behaviour: EnumeTileBehaviour::Empty
        };
    }
}

// -- SYSTEM -----------------------------------------------------------------

// -- tile --

fn s_create_tile(
    mut commands: Commands,
    mut r_grid : ResMut<ResCurrentLevelGrid>,
    r_collection_tile: Res<ResCollectionTile>,
    r_current_level: Res<ResCurrentLevel>,
    mut e_event_tile_creation_asked: EventReader<EventTileCreationAsked>,
    mut e_event_tile_validation_asked: EventWriter<EventTileValidationAsked>,
){
    for e in e_event_tile_creation_asked.read() {
        let tile = &r_collection_tile.tiles[e.tile_idx];
        let x = e.grid_position.x;
        let z = e.grid_position.z;

        remove_tile(&mut commands, &mut r_grid, x, z);

        let entity_commands = commands.spawn(
            (
                BundleTile{
                    model: SceneBundle {
                        scene: tile.tile_model.clone(),
                        transform: e.tile_transform.clone(),
                        ..default()
                    }, 
                    tile_id: tile.tile_id.clone(),
                    grid_position: GridPosition{x, z}
                },
                MarkerTileOnLevel,
                LevelUid{uid: r_current_level.level_uid},
            ),
        );
        let entity = entity_commands.id();

        r_grid.level_grid[x][z] = LevelGridTile{
            tile_id: Some(tile.tile_id),
            tile_behaviour: tile.tile_behaviour,
            tile_entity: Some(entity)
        };

        e_event_tile_validation_asked.send(
            EventTileValidationAsked{
                grid_position: e.grid_position.clone(),
                validation_payload: TileValidationPayload::AddedTile,
            }
        );
    }
}

fn s_remove_tile(
    mut commands: Commands,
    mut r_grid : ResMut<ResCurrentLevelGrid>,
    mut e_event_tile_removal_asked: EventReader<EventTileRemovalAsked>,
    mut e_event_tile_validation_asked: EventWriter<EventTileValidationAsked>,
) {
    for e in e_event_tile_removal_asked.read() {

        let x = e.grid_position.x;
        let z = e.grid_position.z;

        remove_tile(&mut commands, &mut r_grid, x, z);

        e_event_tile_validation_asked.send(
            EventTileValidationAsked{
                grid_position: e.grid_position.clone(),
                validation_payload: TileValidationPayload::RemovedTile,
            }
        );
    }
}

// -- hedeghog --

fn s_create_hedgehog(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    r_hedgehog: Res<HedgehogAssets>,
    mut r_grid : ResMut<ResCurrentLevelGrid>,
    r_current_level: Res<ResCurrentLevel>,
    mut e_event_hedgehog_creation_asked: EventReader<EventHedgehogCreationAsked>,
    mut e_event_tile_validation_asked: EventWriter<EventTileValidationAsked>,
){
    for e in e_event_hedgehog_creation_asked.read() {

        let x = e.grid_position.x;
        let z = e.grid_position.z;

        let hedgehog_material = materials.add(
            StandardMaterial{
                base_color_texture: Some(r_hedgehog.sprite_idle.clone()),
                alpha_mode: AlphaMode::Mask(0.5),
                ..Default::default()
            }
        );

        remove_hedgehog(&mut commands, &mut r_grid, x, z);

        let entity_commands = commands.spawn(
            (
                BundleHedgehog {
                    model: PbrBundle {
                        mesh: meshes.add(Mesh::from(Plane3d{normal: Dir3::Y, half_size: Vec2{x:1.0, y:1.0,}})),
                        material: hedgehog_material,
                        transform: e.hedgehog_transform.clone(),
                        ..Default::default()
                    },
                    grid_position: GridPosition{x, z},
                    hedgehog_type: HedgehogType::HedegehogeTypeStandard,
                }, 
                MarkerHedgehogOnLevel,
                LevelUid{uid: r_current_level.level_uid},
            )
        );

        let entity = entity_commands.id();

        r_grid.hedgehog_grid[x][z] = LevelGridHedgehog{
            hedgehog_behaviour: EnumHedgehogOnGrid::HedgehogAlive,
            hedgehog_entity: Some(entity),
            hedgehog_tile: Some(HedgehogType::HedegehogeTypeStandard),
        };

        e_event_tile_validation_asked.send(
            EventTileValidationAsked{
                grid_position: e.grid_position.clone(),
                validation_payload: TileValidationPayload::AddedHedgehog,
            }
        );
    }
}

fn s_remove_hedgehog(
    mut commands: Commands,
    mut r_grid : ResMut<ResCurrentLevelGrid>,
    mut e_event_hedgehog_removal_asked: EventReader<EventHedgehogRemovalAsked>,
    mut e_event_tile_validation_asked: EventWriter<EventTileValidationAsked>,
) {
    for e in e_event_hedgehog_removal_asked.read() {
        let x = e.grid_position.x;
        let z = e.grid_position.z;

        remove_hedgehog(&mut commands, &mut r_grid, x, z);

        e_event_tile_validation_asked.send(
            EventTileValidationAsked{
                grid_position: e.grid_position.clone(),
                validation_payload: TileValidationPayload::RemovedHedgehog,
            }
        );
    }
}

fn s_validate_level_edition(
    mut commands: Commands,
    mut e_event_tile_validation_asked: EventReader<EventTileValidationAsked>,
    mut e_event_level_edited: EventWriter<EventLevelEdidted>,
    mut r_grid : ResMut<ResCurrentLevelGrid>,
) {
    for e in e_event_tile_validation_asked.read(){

        let x = e.grid_position.x;
        let z = e.grid_position.z;
        let tile_behaviour = r_grid.level_grid[x][z].tile_behaviour;

        match e.validation_payload{
            TileValidationPayload::RemovedTile => {
                remove_hedgehog(&mut commands, &mut r_grid, x, z);
            },
            TileValidationPayload::AddedTile => {
                match tile_behaviour {
                    EnumeTileBehaviour::TileBFloor => {}
                    _ => {
                        remove_hedgehog(&mut commands, &mut r_grid, x, z);
                    }
                }
            }
            _ => {}
        }
    }
    e_event_level_edited.send(EventLevelEdidted);
}
