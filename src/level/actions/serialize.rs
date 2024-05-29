use crate::level::definition::level_definition::{
    LevelDescription, LevelDescriptionTile, LevelGrid, LEVEL_DEFAULT_SIZE,
};
use bevy::{prelude::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};

#[derive(Resource, Default)]
pub struct RessourceTypeRegister {
    type_register: AppTypeRegistry,
}

const FILE_PATH: &str = "levels_desc/test_level.ron";

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginSerialize;

impl Plugin for PluginSerialize {
    fn build(&self, app: &mut App) {
        app.insert_resource(RessourceTypeRegister::default())
            .add_systems(PostStartup, register_type_registry)
            .add_systems(Update, save_to_file_level_desc);
    }
}

fn register_type_registry(world: &mut World) {
    let res = world.resource::<AppTypeRegistry>().clone();
    let mut type_register = world.resource_mut::<RessourceTypeRegister>();
    type_register.type_register = res;
}

fn save_to_file_level_desc(
    r_type_register: Res<RessourceTypeRegister>,
    r_grid: Res<LevelGrid>,
    r_keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !r_keyboard_input.just_pressed(KeyCode::KeyX) {
        return;
    }

    const ARRAY_INIT_VALUE: LevelDescriptionTile = LevelDescriptionTile {
        tile: None,
        hedgehog: None,
    };
    let mut grid = [[ARRAY_INIT_VALUE; LEVEL_DEFAULT_SIZE]; LEVEL_DEFAULT_SIZE];
    for x in 0..=LEVEL_DEFAULT_SIZE - 1 {
        for z in 0..=LEVEL_DEFAULT_SIZE - 1 {
            grid[x][z] = LevelDescriptionTile {
                tile: r_grid.level_grid[x][z].tile_id,
                hedgehog: r_grid.hedgehog_grid[x][z].hedgehog_tile,
            }
        }
    }

    let mut scene_world = World::new();
    let type_registry = r_type_register.type_register.clone();
    scene_world.insert_resource(type_registry);
    scene_world.spawn(LevelDescription { level_grid: grid });
    let scene = DynamicScene::from_world(&scene_world);

    // Scenes can be serialized like this:
    let type_registry = r_type_register.type_register.clone();
    let serialized_scene_ron = scene.serialize_ron(&type_registry).unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene_ron.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}

fn load_from_file_level_desc(mut commands: Commands, asset_server: Res<AssetServer>) {}

// TODO: bon, tout revoir en utilisant des Dynamic (ou non) Scene.
// Mais utiliser des scènes...
