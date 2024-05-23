use bevy::prelude::*;
use  crate::common::common::GridPosition;
use crate::common::asset_loader::{SceneAssets, load_scene_assets};

pub const TILE_SIZE: f32 = 2.0;
pub const TILE_WIDTH: f32 = 0.3;

#[derive(Reflect, Component, Default, Debug, Clone, Copy)]
pub enum EnumTilesId {
    #[default]
    TileIdFloor,
    TileIdFire,
    TileIdWater,
    TileIdExit,
    TileIdArmoire,
    TileIdTable1,
    TileIdTable2,
    TileIdWallCorner,
    TileIdWall,
    TileIdDesk,
}

#[derive(Reflect, Component, Default, Debug, Clone, Copy)]
pub enum EnumeTileBehaviour {
    #[default]
    TileBFloor,
    TileBFire,
    TileBWater,
    TileBExit,
    TileBObstacle,
    Empty,
}

pub struct DefinitionTile{
    pub tile_id: EnumTilesId,
    pub tile_model: Handle<Scene>,
    pub tile_behaviour: EnumeTileBehaviour,
}

#[derive(Resource, Default)]
pub struct ResCollectionTile{
    pub tiles: Vec<DefinitionTile>,
}

#[derive(Bundle, Default)]
pub struct BundleTile{
    pub model: SceneBundle,
    pub tile_id: EnumTilesId,
    pub grid_position: GridPosition, 
}

#[derive(Component)]
pub struct MarkerTileOnLevel;

pub struct PluginTiles;

impl Plugin for PluginTiles{
    fn build(&self, app: &mut App){
        app.init_resource::<ResCollectionTile>()
            .add_systems(Startup, build_res_collection_tiles.after(load_scene_assets));
    }
}

fn build_res_collection_tiles(
    mut res_collection_tiles: ResMut<ResCollectionTile>,
    scene_assets: Res<SceneAssets>,
){
    res_collection_tiles.tiles.push(
        DefinitionTile {
            tile_id: EnumTilesId::TileIdFloor,
            tile_model: scene_assets.tile_floor.clone(),
            tile_behaviour: EnumeTileBehaviour::TileBFloor,
        }
    );
    res_collection_tiles.tiles.push(
        DefinitionTile {
            tile_id: EnumTilesId::TileIdFire,
            tile_model: scene_assets.tile_fire.clone(),
            tile_behaviour: EnumeTileBehaviour::TileBFire,
        }
    );
    res_collection_tiles.tiles.push(
        DefinitionTile {
            tile_id: EnumTilesId::TileIdWater,
            tile_model: scene_assets.tile_water.clone(),
            tile_behaviour: EnumeTileBehaviour::TileBWater,
        }
    );
    res_collection_tiles.tiles.push(
        DefinitionTile {
            tile_id: EnumTilesId::TileIdExit,
            tile_model: scene_assets.tile_exit.clone(),
            tile_behaviour: EnumeTileBehaviour::TileBExit,
        }
    );
    res_collection_tiles.tiles.push(
        DefinitionTile {
            tile_id: EnumTilesId::TileIdArmoire,
            tile_model: scene_assets.tile_armoire.clone(),
            tile_behaviour: EnumeTileBehaviour::TileBObstacle,
        }
    );
    res_collection_tiles.tiles.push(
        DefinitionTile {
            tile_id: EnumTilesId::TileIdTable1,
            tile_model: scene_assets.tile_table_1.clone(),
            tile_behaviour: EnumeTileBehaviour::TileBObstacle,
        }
    );
    res_collection_tiles.tiles.push(
        DefinitionTile {
            tile_id: EnumTilesId::TileIdTable2,
            tile_model: scene_assets.tile_table_2.clone(),
            tile_behaviour: EnumeTileBehaviour::TileBObstacle,
        }
    );
    res_collection_tiles.tiles.push(
        DefinitionTile {
            tile_id: EnumTilesId::TileIdWallCorner,
            tile_model: scene_assets.tile_wall_corner.clone(),
            tile_behaviour: EnumeTileBehaviour::TileBObstacle,
        }
    );
    res_collection_tiles.tiles.push(
        DefinitionTile {
            tile_id: EnumTilesId::TileIdWall,
            tile_model: scene_assets.tile_wall.clone(),
            tile_behaviour: EnumeTileBehaviour::TileBObstacle,
        }
    );
    res_collection_tiles.tiles.push(
        DefinitionTile {
            tile_id: EnumTilesId::TileIdDesk,
            tile_model: scene_assets.title_desk.clone(),
            tile_behaviour: EnumeTileBehaviour::TileBObstacle,
        }
    );
}
