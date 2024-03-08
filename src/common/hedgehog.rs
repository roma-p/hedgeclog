use bevy::prelude::*;
pub struct PluginHedghog;

impl Plugin for PluginHedghog{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_hedgehog);
    }
}

fn spawn_hedgehog(mut commands: Commands) {
}
