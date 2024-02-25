use bevy::prelude::*;

pub struct PluginDebug;

impl Plugin for PluginDebug{
    fn build(&self, app: &mut App){
        app.add_systems(Update, print_hedgehog_position);
    }
}

fn print_hedgehog_position(mut query: Query<(Entity, &Transform)>){
    for (entity, transform) in query.iter_mut() {
        info!("entity {:?} is at position {:?},", entity, transform)
    }
}
