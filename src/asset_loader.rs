use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub tile_floor: Handle<Scene>,
    pub tile_water: Handle<Scene>,
    pub tile_fire: Handle<Scene>,
}

pub struct PluginAssetLoader;

impl Plugin for PluginAssetLoader{
    fn build(&self, app: &mut App){
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
) {

    *scene_assets = SceneAssets {
        tile_floor: asset_server.load("tiles/tile_floor.gltf#Scene0"),
        tile_water: asset_server.load("tiles/tile_water.gltf#Scene0"),
        tile_fire: asset_server.load("tiles/tile_fire.gltf#Scene0"),
    };
}
