use bevy::prelude::*;
use crate::common::common::GridPosition;

#[derive(Resource, Debug, Default)]
pub struct ModeTileLocalBuffer {
    pub selected_idx: usize,
    pub hover_tile_grid_position: Option<GridPosition>
}

pub struct PluginEditorModeTileCommon;

impl Plugin for PluginEditorModeTileCommon{
    fn build(&self, app: &mut App){
        app
            .insert_resource(ModeTileLocalBuffer::default());
    }
}

