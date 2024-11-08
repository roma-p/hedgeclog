use std::usize;
use bevy::prelude::*;
use bevy::utils::Uuid;

use crate::level::definition::tiles::*;
use crate::level::definition::hedgehog::*;

// -- CONSTS -----------------------------------------------------------------

pub const LEVEL_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const TRANSLATION_LEVEL_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const TRANSLATION_DEFAULT_CAMERA_SHIFT: Vec3 = Vec3::new(9.0, 12.0, 9.0);
pub const LEVEL_DEFAULT_SIZE: usize = 10;

// -- COMPONENTS -------------------------------------------------------------

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct GridPosition {
    pub x : usize,
    pub z : usize,
}

// --- "Logic" representation of the level. ---
// Used by gameplay code to know what should happen on player action.

#[derive(Debug, Default, Clone, Copy)]
pub struct LevelGridTile {
    pub tile_id: Option<EnumTilesId>,
    pub tile_entity: Option<Entity>,
    pub tile_behaviour: EnumeTileBehaviour,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LevelGridHedgehog {
    pub hedgehog_behaviour: EnumHedgehogOnGrid, // TODO: rename this.
    pub hedgehog_entity: Option<Entity>,
    pub hedgehog_tile: Option<HedgehogType>,
}


// --- "Description" a level. ---
// a sum up of the level that is used to serialize / deserialie levels.

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LevelDescription {
    pub level_grid: [[LevelDescriptionTile; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
    pub uuid: Uuid
}

#[derive(Reflect, Clone, Copy)]
pub struct LevelDescriptionTile {
    pub tile: Option<EnumTilesId>,
    pub hedgehog: Option<HedgehogType>,
    // pub object: bool // TODO: LATER.
}

#[derive(Component, Default, Debug, Clone)]
pub struct LevelUid{
    pub uid: Option<Uuid>,
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct LevelGrid {
    pub level_grid: [[LevelGridTile; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
    pub hedgehog_grid: [[LevelGridHedgehog; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
}

// -- RESSOURCES -------------------------------------------------------------

#[derive(Resource, Default)]
pub struct ResCurrentLevel{
    pub level_uid: Option<Uuid>,
    pub level_entity: Option<Entity>,
    // TODO -> level grid ?
}

#[derive(Resource, Debug, Default)]
pub struct ResCurrentLevelGrid {
    pub level_grid: [[LevelGridTile; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
    pub hedgehog_grid: [[LevelGridHedgehog; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
}

pub struct PluginLevelDefinition;

impl Plugin for PluginLevelDefinition{
    fn build(&self, app: &mut App){
        app
            .insert_resource(ResCurrentLevelGrid::default())
            .insert_resource(ResCurrentLevel::default())
            .register_type::<LevelDescription>()
            .register_type::<LevelDescriptionTile>()
            .register_type::<EnumTilesId>()
            .register_type::<HedgehogType>();


    }
}

