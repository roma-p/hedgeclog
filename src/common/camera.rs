use bevy::{prelude::*, render::camera::ScalingMode};

pub struct PluginCamera;

impl Plugin for PluginCamera {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle{
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(15.0),
            ..default()
        }.into(),
        transform: Transform::from_xyz(9.0, 9.0, 9.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
