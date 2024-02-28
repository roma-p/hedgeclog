use bevy::prelude::*;

use crate::common::asset_loader::SceneAssets;

pub const LEVEL_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);

// -- COMPONENENTS : GRID INFO -----------------------------------------------

#[derive(Component, Debug, Clone)]
pub struct GridPosition {
    pub value: IVec2
}

// -- BUNDLE : TILES ---------------------------------------------------------

#[derive(Bundle)]
pub struct BundleTile{
    pub model: SceneBundle,
    pub grid_position: GridPosition,
}

// tiles that actually compose the level
#[derive(Bundle)]
pub struct BundleTileLevel{
    pub tile: BundleTile,
}


pub struct PluginLevel;

impl Plugin for PluginLevel{
    fn build(&self, app: &mut App){
        // app.add_systems(PostStartup, spawn_level_tile_from_selector);
    }
}

// fn query_scene_entities(
//     commands: Commands,
//     scene_spawner: Res<SceneSpawner>,
//     scene_handles: Query<(Entity, &Handle<Scene>)>,
// ) {
//     for (entity, scene_handle) in scene_handles.iter() {
//         // Spawn the scene into the world
//         let scene_instance = scene_spawner.spawn(scene_handle.clone());
//         
//         // Query entities in the scene
//         for entity in scene_spawner.iter_instance_entities(scene_instance) {
//             // Perform operations on the entities
//             println!("Entity in scene: {:?}", entity);
//         }
//     }
// }
