use std::usize;
use bevy::prelude::*;
use crate::level::definition::tiles::*;
use crate::level::definition::hedgehog::*;

pub const LEVEL_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const TRANSLATION_LEVEL_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const TRANSLATION_DEFAULT_CAMERA_SHIFT: Vec3 = Vec3::new(9.0, 12.0, 9.0);
pub const LEVEL_DEFAULT_SIZE: usize = 10;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct GridPosition {
    pub x : usize,
    pub z : usize,
}

#[derive(Resource, Debug, Default)]
pub struct LevelGrid {
    pub level_grid: [[LevelGridTile; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
    pub hedgehog_grid: [[LevelGridHedgehog; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
}

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


#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LevelDescription {
    pub level_grid: [[LevelDescriptionTile; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
}

#[derive(Reflect, Clone, Copy)]
pub struct LevelDescriptionTile {
    pub tile: Option<EnumTilesId>,
    pub hedgehog: Option<HedgehogType>,
    // pub object: bool // TODO: LATER.
}


pub struct PluginLevelDefinition;

impl Plugin for PluginLevelDefinition{
    fn build(&self, app: &mut App){
        app
            .insert_resource(LevelGrid::default());
    }
}

