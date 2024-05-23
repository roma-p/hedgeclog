use bevy::prelude::*;

#[derive(Resource, Debug, Default, Clone)]
pub struct SceneAssets {
    pub tile_floor: Handle<Scene>,
    pub tile_fire: Handle<Scene>,
    pub tile_water: Handle<Scene>,
    pub tile_armoire: Handle<Scene>,
    pub tile_exit: Handle<Scene>,
    pub tile_table_1: Handle<Scene>,
    pub tile_table_2: Handle<Scene>,
    pub tile_wall_corner: Handle<Scene>,
    pub tile_wall_angle: Handle<Scene>,
    pub tile_wall: Handle<Scene>,
    pub title_desk: Handle<Scene>,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct HedgehogAssets {
    pub sprite_idle: Handle<Image>,
}


pub struct PluginAssetLoader;

impl Plugin for PluginAssetLoader{
    fn build(&self, app: &mut App){
        app
            .init_resource::<SceneAssets>()
            .init_resource::<HedgehogAssets>()
            .add_systems(Startup, (load_scene_assets, load_hedgehog_assets));
    }
}

fn load_hedgehog_assets(
    mut hedgehog_assets: ResMut<HedgehogAssets>,
    asset_server: Res<AssetServer>,
) {
    *hedgehog_assets = HedgehogAssets {
        sprite_idle: asset_server.load("hedgehog/hed_schema_1.1.png"),
    };
}

pub fn load_scene_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
) {

    *scene_assets = SceneAssets {
        tile_water: asset_server.load("tiles/tile_water.gltf#Scene0"),
        tile_armoire: asset_server.load("tiles/tile_armoire.gltf#Scene0"),
        tile_exit: asset_server.load("tiles/tile_exit.gltf#Scene0"),
        tile_fire: asset_server.load("tiles/tile_fire.gltf#Scene0"),
        tile_floor: asset_server.load("tiles/tile_floor.gltf#Scene0"),
        tile_table_1: asset_server.load("tiles/tile_table_1.gltf#Scene0"),
        tile_table_2: asset_server.load("tiles/tile_table_2.gltf#Scene0"),
        tile_wall: asset_server.load("tiles/tile_wall.gltf#Scene0"),
        tile_wall_angle: asset_server.load("tiles/tile_wall_angle.gltf#Scene0"),
        tile_wall_corner: asset_server.load("tiles/tile_wall_corner.gltf#Scene0"),
        title_desk: asset_server.load("tiles/title_desk.gltf#Scene0"),
    };
}
