use bevy::prelude::*;
pub struct PluginHedghog;

use crate::{
    common::level::{GridPosition},
    common::movement::Velocity
};

impl Plugin for PluginHedghog{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_hedgehog);
    }
}

fn spawn_hedgehog(mut commands: Commands) {
    commands.spawn(
        (
            SpatialBundle::default(),
            GridPosition{
                value: IVec2::new(0, 0)
            },
            Velocity{
                value: Vec3::new(0.0, 0.0, 0.0)
            },
        )
    );
}
