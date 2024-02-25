use bevy::prelude::*;

pub struct PluginMovement;

#[derive(Component, Debug)]
pub struct GridPosition {
    pub value: IVec2
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3
}


impl Plugin for PluginMovement{
    fn build(&self, app: &mut App){
        app.add_systems(Update, move_hedgehog);
    }
}

fn move_hedgehog(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

