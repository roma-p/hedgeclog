use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component, Debug, Clone)]
pub struct FloatComponent(f32);

// tiles that actually compose the level
#[derive(Bundle)]
pub struct BundleCameraInfo{
    pub transform: Transform,
    pub scaling_mode:FloatComponent,
}

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
        // transform: Transform::from_xyz(9.0, 9.0, 9.0)
        transform: Transform::from_xyz(-1000.0, -1000.0, -1000.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
