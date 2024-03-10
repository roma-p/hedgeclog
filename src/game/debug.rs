use bevy::prelude::*;
use crate::config::{StateGlobal, StateLevelLoaded};
use crate::editor::common::StateEditorLoaded;

pub struct PluginDebug;

impl Plugin for PluginDebug{
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(StateGlobal::Editor), print_mode_editor);
        app.add_systems(OnEnter(StateGlobal::Game), print_mode_game);
        app.add_systems(OnEnter(StateEditorLoaded::Loading), print_loading_editor);
        app.add_systems(OnEnter(StateEditorLoaded::Loaded), print_loaded_editor);
        app.add_systems(OnEnter(StateLevelLoaded::Loading), print_loading_level);
        app.add_systems(OnEnter(StateLevelLoaded::Loaded),  print_loaded_level);
    } 
}

fn print_mode_editor() {
    info!("Mode changed: EDITOR.");
}

fn print_mode_game() {
    info!("Mode changed: GAME.");
}

fn print_loading_editor() {
    info!("Loading editor...");
}

fn print_loaded_editor() {
    info!("Loaded editor.");
}

fn print_loading_level() {
    info!("Loading level...");
}

fn print_loaded_level() {
    info!("Loaded level.");
}

// fn print_hedgehog_position(mut query: Query<(Entity, &Transform)>){
//     for (entity, transform) in query.iter_mut() {
//         info!("entity {:?} is at position {:?},", entity, transform)
//     }
// }
