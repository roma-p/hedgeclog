use bevy::prelude::*;
use crate::config::{StateGlobal, StateEditorLoaded};

pub struct PluginDebug;

impl Plugin for PluginDebug{
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(StateGlobal::Editor), print_mode_editor);
        app.add_systems(OnEnter(StateGlobal::Game), print_mode_game);
        app.add_systems(OnEnter(StateEditorLoaded::Loading), print_loading_editor);
        app.add_systems(OnEnter(StateEditorLoaded::Loaded), print_loaded_editor);
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

// fn print_hedgehog_position(mut query: Query<(Entity, &Transform)>){
//     for (entity, transform) in query.iter_mut() {
//         info!("entity {:?} is at position {:?},", entity, transform)
//     }
// }
