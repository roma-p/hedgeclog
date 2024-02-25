
use std::ops::Add;

use bevy::{prelude::*, transform::commands};

use crate::{
    asset_loader::SceneAssets, movement::GridPosition
};

const LEVEL_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Bundle)]
pub struct BundleTile{
    model: SceneBundle,
    grid_position: GridPosition,
}

// singleton of the tile display to edit level.
#[derive(Bundle)]
pub struct BundleTileSelector{
    model: SceneBundle,
}

// used to detect mouse cursor position with level editor.
#[derive(Component)]
struct GroundPlane;

pub struct PluginLevel;

impl Plugin for PluginLevel{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup, spawn_tile);
    }
}

fn spawn_selector_entity(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn(BundleTile{
        model: SceneBundle {
            scene: scene_assets.tile_floor.clone(),
            transform: Transform::from_translation(LEVEL_ORIGIN),
            ..default()
        }, 
        grid_position: GridPosition{
            value: IVec2::new(0, 0)
        }
    });
}

fn spawn_tile(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn(BundleTile{
        model: SceneBundle {
            scene: scene_assets.tile_water.clone(),
            transform: Transform::from_translation(LEVEL_ORIGIN),
            ..default()
        }, 
        grid_position: GridPosition{
            value: IVec2::new(0, 0)
        }
    });
    commands.spawn(BundleTile{
        model: SceneBundle {
            scene: scene_assets.tile_fire.clone(),
            transform: Transform::from_translation(LEVEL_ORIGIN.add(Vec3::new(2.0, 0.0, 0.0))),
            ..default()
        }, 
        grid_position: GridPosition{
            value: IVec2::new(0, 0)
        }
    });
}


// a system spawn_current_asset
// a system select_asset: spawn it but move it...
