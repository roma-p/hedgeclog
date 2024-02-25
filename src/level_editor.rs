use bevy::prelude::*;

pub struct PluginLevelEditor;

impl Plugin for PluginLevelEditor{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup, edit_level);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum StateSelectedTile {
    tile_floor,
    tile_water,
    tile_fire,
}

fn edit_level(mut commands: Commands) {
}
