use bevy::prelude::*;
use crate::level::definition::level_definition::{
    LevelGrid, LEVEL_DEFAULT_SIZE,
    LevelDescriptionTile, LevelDescription
};

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginSerialize;

impl Plugin for PluginSerialize{
    fn build(&self, app: &mut App){
    }
}

fn generate_level_description(
    mut commands: Commands,
    r_grid: Res<LevelGrid>,
) {
    fn_generate_level_description(&mut commands, &r_grid);
}

fn fn_generate_level_description(
    commands: &mut Commands,
    r_grid: &Res<LevelGrid>,
){
    const ARRAY_INIT_VALUE:LevelDescriptionTile = LevelDescriptionTile{
        tile: None,
        hedgehog: None
    };
    let mut grid = [[ARRAY_INIT_VALUE; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE];
    for x in 0..=LEVEL_DEFAULT_SIZE {
        for z in 0..=LEVEL_DEFAULT_SIZE {
            grid[x][z] = LevelDescriptionTile{
                tile: r_grid.level_grid[x][z].tile_id,
                hedgehog: r_grid.hedgehog_grid[x][z].hedgehog_tile
            }
        }
    }
    commands.spawn(
        LevelDescription {level_grid: grid}
    );
}
