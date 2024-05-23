
use std::collections::btree_map::Range;

use bevy::math::bool;
use bevy::prelude::*;

use crate::config::{StateLevelLoaded, LEVEL_DEFAULT_SIZE, StateUserInputAllowed};
use crate::common::common::GridPosition;
use crate::common::tiles::{
    BundleTile,
    EnumeTileBehaviour,
    EnumTilesId,
    ResCollectionTile,
    MarkerTileOnLevel, 
};
use crate::common::hedgehog::{
    EnumHedgehogOnGrid,
    BundleHedgehog,
    MarkerHedgehogOnLevel,  // TODO: move this and tile to level.rs.
};
use crate::common::asset_loader::HedgehogAssets;

// CONST / ENUM / EVENT / COMPONENT / RESSOURCE ------------------------------
 
pub const LEVEL_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);

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

#[derive(Debug, Default, Clone, Copy)]
pub struct LevelGridTile {
    pub tile_behaviour: EnumeTileBehaviour,
    pub tile_entity: Option<Entity>
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LevelGridHedgehog {
    pub hedgehog_behaviour: EnumHedgehogOnGrid,
    pub hedgehog_entity: Option<Entity>
}

#[derive(Resource, Debug, Default)]
pub struct LevelGrid {
    pub level_grid: [[LevelGridTile; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
    pub hedgehog_grid: [[LevelGridHedgehog; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
}

#[derive(Component)]
struct MarkerEditorGUI;

// marker component
#[derive(Component)]
pub struct MarkerTextLoadingLevel;

#[derive(Reflect)]
pub struct LevelDescriptionTile {
    tile: Option<EnumTilesId>,
    hedgehog: bool,  // CHANGE THIS
    object: bool // LATER.
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LevelDescription {
    pub level_grid: [[LevelDescriptionTile; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
}



// -- PLUGIN -----------------------------------------------------------------

pub struct PluginLevel;

impl Plugin for PluginLevel{
    fn build(&self, app: &mut App){
        app
            .insert_resource(LevelGrid::default())
            .add_event::<EventTileCreationAsked>()
            .add_event::<EventTileRemovalAsked>()
            .add_event::<EventHedgehogCreationAsked>()
            .add_event::<EventHedgehogRemovalAsked>()
            .add_event::<EventTileValidationAsked>()
            .add_event::<EventLevelEdidted>()
            .add_systems(
                PostStartup,
                level_loading_prepare.run_if(in_state(StateLevelLoaded::NotLoaded))
                
            )
            .add_systems(OnEnter(StateLevelLoaded::Loading), level_loading_load)
            .add_systems(
                Update,
                (
                    create_tile.run_if(on_event::<EventTileCreationAsked>()),
                    remove_tile.run_if(on_event::<EventTileRemovalAsked>()),
                    create_hedgehog.run_if(on_event::<EventHedgehogCreationAsked>()),
                    remove_hedgehog.run_if(on_event::<EventHedgehogRemovalAsked>()),
                    validate_level_edition.run_if(on_event::<EventTileValidationAsked>())
                )
            );
    }
}

// -- FUNCTIONS --------------------------------------------------------------

fn fn_remove_hedgehog(
    commands: &mut Commands,
    r_grid: &mut ResMut<LevelGrid>,
    x: usize, z: usize,
){
    if let Some(entity) = r_grid.hedgehog_grid[x][z].hedgehog_entity{
        commands.entity(entity).despawn();
        r_grid.hedgehog_grid[x][z] = LevelGridHedgehog{
            hedgehog_entity: None,
            hedgehog_behaviour: EnumHedgehogOnGrid::Empty
        };
    }
}

fn fn_remove_tile(
    commands: &mut Commands,
    r_grid: &mut ResMut<LevelGrid>,
    x: usize, z: usize,
){
    if let Some(tile_entity) = r_grid.level_grid[x][z].tile_entity{
        commands.entity(tile_entity).despawn();
        r_grid.level_grid[x][z] = LevelGridTile{
            tile_entity: None,
            tile_behaviour: EnumeTileBehaviour::Empty
        };
    }
}

// ZOOM --

pub enum ZoomLevel {
    REALLYSMALL = 6,
    SMALL = 10,
    NORMAL = 12,
    BIG = 15,
    REALLYBIG = 20,
}

impl ZoomLevel {
    pub fn unzoom(&self) -> Option<ZoomLevel> {
        match self {
            ZoomLevel::REALLYSMALL => Some(ZoomLevel::SMALL),
            ZoomLevel::SMALL => Some(ZoomLevel::NORMAL),
            ZoomLevel::NORMAL => Some(ZoomLevel::BIG),
            ZoomLevel::BIG => Some(ZoomLevel::REALLYBIG),
            ZoomLevel::REALLYBIG => Some(ZoomLevel::REALLYBIG),
        }
    }

    pub fn zoom(&self) -> Option<ZoomLevel> {
        match self {
            ZoomLevel::REALLYBIG => Some(ZoomLevel::BIG),
            ZoomLevel::BIG => Some(ZoomLevel::NORMAL),
            ZoomLevel::NORMAL => Some(ZoomLevel::SMALL),
            ZoomLevel::SMALL => Some(ZoomLevel::REALLYSMALL),
            ZoomLevel::REALLYSMALL => Some(ZoomLevel::REALLYSMALL),
        }
    }

    pub fn get_from_i32(value: i32) -> Option<ZoomLevel> {

        const I32_REALLYSMALL: i32 = ZoomLevel::REALLYSMALL as i32;
        const I32_SMALL: i32 = ZoomLevel::SMALL as i32;
        const I32_NORMAL: i32 = ZoomLevel::NORMAL as i32;
        const I32_BIG: i32 = ZoomLevel::BIG as i32;
        const I32_REALLYBIG: i32 = ZoomLevel::REALLYBIG as i32;

        match value {
            I32_REALLYSMALL => Some(ZoomLevel::REALLYSMALL),
            I32_SMALL => Some(ZoomLevel::SMALL),
            I32_NORMAL=> Some(ZoomLevel::NORMAL),
            I32_BIG => Some(ZoomLevel::BIG),
            I32_REALLYBIG => Some(ZoomLevel::REALLYBIG),
            _ => None
        }
    }
}


// -- SYSTEM -----------------------------------------------------------------

// -- loading --

fn level_loading_prepare(
    mut commands: Commands,
    mut state_level_loaded: ResMut<NextState<StateLevelLoaded>>,
) {
    commands.spawn(
        (
            TextBundle::from_section(
                "Loading level...",
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
            MarkerTextLoadingLevel,
        )
    );
    state_level_loaded.set(StateLevelLoaded::Loading);
}

fn level_loading_load(
    mut commands: Commands,
    mut s_level_loaded: ResMut<NextState<StateLevelLoaded>>,
    mut r_level_grid: ResMut<LevelGrid>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
    entity: Query<Entity, With <MarkerTextLoadingLevel>>
) {
    // const ARRAY_REPEAT_VALUE:EnumeTileBehaviour = EnumeTileBehaviour::Empty;
    // FIXME: WAIT WILL IT BE THE SAME REFERENCE?
    const ARRAY_REPEAT_VALUE:LevelGridTile = LevelGridTile{
        tile_behaviour: EnumeTileBehaviour::Empty,
        tile_entity: None
    };
    r_level_grid.level_grid = [[ARRAY_REPEAT_VALUE; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE];
    commands.entity(entity.single()).despawn();
    s_level_loaded.set(StateLevelLoaded::Loaded);
    s_user_input_allowed.set(StateUserInputAllowed::Allowed);
}

// -- tile --

fn create_tile(
    mut commands: Commands,
    mut r_grid : ResMut<LevelGrid>,
    r_collection_tile: Res<ResCollectionTile>,
    mut e_event_tile_creation_asked: EventReader<EventTileCreationAsked>,
    mut e_event_tile_validation_asked: EventWriter<EventTileValidationAsked>,
){
    for e in e_event_tile_creation_asked.read() {
        let tile = &r_collection_tile.tiles[e.tile_idx];
        let x = e.grid_position.x;
        let z = e.grid_position.z;

        fn_remove_tile(&mut commands, &mut r_grid, x, z);

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
            ),
        );
        r_grid.level_grid[x][z] = LevelGridTile{
            tile_behaviour: tile.tile_behaviour,
            tile_entity: Some(entity_commands.id())
        };
        e_event_tile_validation_asked.send(
            EventTileValidationAsked{
                grid_position: e.grid_position.clone(),
                validation_payload: TileValidationPayload::AddedTile,
            }
        );
    }
}

fn remove_tile(
    mut commands: Commands,
    mut r_grid : ResMut<LevelGrid>,
    mut e_event_tile_removal_asked: EventReader<EventTileRemovalAsked>,
    mut e_event_tile_validation_asked: EventWriter<EventTileValidationAsked>,
) {
    for e in e_event_tile_removal_asked.read() {

        let x = e.grid_position.x;
        let z = e.grid_position.z;

        fn_remove_tile(&mut commands, &mut r_grid, x, z);

        e_event_tile_validation_asked.send(
            EventTileValidationAsked{
                grid_position: e.grid_position.clone(),
                validation_payload: TileValidationPayload::RemovedTile,
            }
        );
    }
}

// -- hedeghog --

fn create_hedgehog(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    r_hedgehog: Res<HedgehogAssets>,
    mut r_grid : ResMut<LevelGrid>,
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

        fn_remove_hedgehog(&mut commands, &mut r_grid, x, z);

        let entity_commands = commands.spawn(
            (
                BundleHedgehog {
                    model: PbrBundle {
                        mesh: meshes.add(Mesh::from(Plane3d{normal: Direction3d::Y})),
                        material: hedgehog_material,
                        transform: e.hedgehog_transform.clone(),
                        ..Default::default()
                    },
                    grid_position: GridPosition{x, z}
                }, 
                MarkerHedgehogOnLevel,
            )
        );

        r_grid.hedgehog_grid[x][z] = LevelGridHedgehog{
            hedgehog_behaviour: EnumHedgehogOnGrid::HedgehogAlive,
            hedgehog_entity: Some(entity_commands.id())
        };

        e_event_tile_validation_asked.send(
            EventTileValidationAsked{
                grid_position: e.grid_position.clone(),
                validation_payload: TileValidationPayload::AddedHedgehog,
            }
        );
    }
}

fn remove_hedgehog(
    mut commands: Commands,
    mut r_grid : ResMut<LevelGrid>,
    mut e_event_hedgehog_removal_asked: EventReader<EventHedgehogRemovalAsked>,
    mut e_event_tile_validation_asked: EventWriter<EventTileValidationAsked>,
) {
    for e in e_event_hedgehog_removal_asked.read() {
        let x = e.grid_position.x;
        let z = e.grid_position.z;

        fn_remove_hedgehog(&mut commands, &mut r_grid, x, z);

        e_event_tile_validation_asked.send(
            EventTileValidationAsked{
                grid_position: e.grid_position.clone(),
                validation_payload: TileValidationPayload::RemovedHedgehog,
            }
        );
    }
}

fn validate_level_edition(
    mut commands: Commands,
    mut e_event_tile_validation_asked: EventReader<EventTileValidationAsked>,
    mut e_event_level_edited: EventWriter<EventLevelEdidted>,
    mut r_grid : ResMut<LevelGrid>,
) {

    for e in e_event_tile_validation_asked.read(){

        let x = e.grid_position.x;
        let z = e.grid_position.z;
        let tile_behaviour = r_grid.level_grid[x][z].tile_behaviour;

        match e.validation_payload{
            TileValidationPayload::RemovedTile => {
                fn_remove_hedgehog(&mut commands, &mut r_grid, x, z);
            },
            TileValidationPayload::AddedTile => {
                match tile_behaviour {
                    EnumeTileBehaviour::TileBFloor => {}
                    _ => {
                        fn_remove_hedgehog(&mut commands, &mut r_grid, x, z);
                    }
                }
            }
            _ => {}
        }
    }
    e_event_level_edited.send(EventLevelEdidted);
}

// -- serialization --

fn generate_level_description(
    mut commands: Commands,
    mut r_grid : ResMut<LevelGrid>,
) {
    // let mut grid: [[LevelDescriptionTile; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE];
    // for x in 0..=LEVEL_DEFAULT_SIZE {
    //     for y in 0..=LEVEL_DEFAULT_SIZE {
    //         grid[x][y] = LevelDescriptionTile{
    //             
    //         }
    //     }
    // }
}

