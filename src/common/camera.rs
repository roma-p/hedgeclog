use bevy::{prelude::*, render::camera::ScalingMode};
use crate::config::{
    StateGlobal,
    StateLevelLoaded,
    TRANSLATION_LEVEL_ORIGIN,
    TRANSLATION_DEFAULT_CAMERA_SHIFT
};

use crate::common::level::ZoomLevel;

use crate::editor::common::StateEditorMode;

#[derive(Bundle, Default)]
pub struct BundleCameraInfo{
    pub projection:OrthographicProjection,
    pub transform: Transform,
}

#[derive(Component)]
pub struct MarkerCamera;

#[derive(Component)]
pub struct MarkerCameraInfoDefault;

#[derive(Event, Debug)]
pub struct EventCameraSnap{
    pub transform: Transform,
    pub projection: OrthographicProjection
}

pub struct PluginCamera;

impl Plugin for PluginCamera {
    fn build(&self, app: &mut App){
        app
            .add_event::<EventCameraSnap>()
            .add_systems(Startup, spawn_camera)
            .add_systems(
                OnEnter(StateLevelLoaded::Loaded), 
                camera_snap_position_default
            )
            .add_systems(
                OnEnter(StateGlobal::Game),
                camera_snap_position_default
                .run_if(in_state(StateLevelLoaded::Loaded))
            )
            .add_systems(
                OnExit(StateEditorMode::Tile),
                camera_snap_position_default
            )
            .add_systems(
                Update,
                snap_camera.run_if(on_event::<EventCameraSnap>())
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        (
            Camera3dBundle{
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::FixedVertical(12.0),
                    ..default()
                }.into(),
                transform: Transform::from_xyz(-1000.0, 9.0, -1000.0)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            MarkerCamera
        )
    );
    commands.spawn(
        (
            BundleCameraInfo{
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::FixedVertical(12.0),
                    ..default()
                }.into(),
                transform: Transform::from_translation(TRANSLATION_LEVEL_ORIGIN
                    .mul_add(Vec3::ONE, TRANSLATION_DEFAULT_CAMERA_SHIFT))
                    .looking_at(TRANSLATION_LEVEL_ORIGIN, Vec3::Y),
                ..default()
            },
            MarkerCameraInfoDefault
        )
    );
}

pub fn camera_snap_position_default(
    camera_info_query: Query<
        (&Transform, &OrthographicProjection,),
        (With <MarkerCameraInfoDefault>, Without<MarkerCamera>)
    >,
    mut e_camera_snap: EventWriter<EventCameraSnap>,
) {
    let (src_transform, src_projection) = camera_info_query.single();
    e_camera_snap.send(
        EventCameraSnap{
            transform: src_transform.clone(),
            projection: src_projection.clone(),
        }
    );
}

pub fn snap_camera(
    mut camera_query: Query<
        (&mut Projection, &mut Transform),
        (With <MarkerCamera>, Without<MarkerCameraInfoDefault>)
    >,
    mut e_camera_snap: EventReader<EventCameraSnap>,
){
    let (mut cam_projection, mut cam_transform) = camera_query.single_mut();

    // only dealing with last event. (or sould it be first? FIXME)
    let last_event = e_camera_snap.read().last().unwrap();
    let target_transform: Transform = last_event.transform.clone();
    let target_projection: OrthographicProjection = last_event.projection.clone();

    *cam_projection = Projection::Orthographic(target_projection.clone());
    *cam_transform = target_transform.clone();
}

pub fn translate_camera(cam_tranform: &mut Mut<Transform>, translation: Vec3) {
    cam_tranform.translation = cam_tranform.translation.mul_add(Vec3::ONE, translation);
}


pub enum ZoomCameraMode {
    Zoom,
    Unzoom
}

pub fn zoom_camera(cam_projection: &mut Mut<Projection>, mode: ZoomCameraMode) {

    let cam_projection_clone = cam_projection.clone();
    let mut tmp_projection: OrthographicProjection = match cam_projection_clone {
        Projection::Orthographic(value) => value,
        _ => panic!()
    };

    let vertical_size: i32 = match tmp_projection.scaling_mode{
        ScalingMode::FixedVertical(value) => value as i32,
        _ => 0
    };

    let current_zoom_level: ZoomLevel = match ZoomLevel::get_from_i32(vertical_size) {
        Some(v) => v,
        None => ZoomLevel::NORMAL
    };

    let new_zoom_level_result: Option<ZoomLevel>;

    match mode {
        ZoomCameraMode::Zoom => new_zoom_level_result = ZoomLevel::zoom(&current_zoom_level),
        ZoomCameraMode::Unzoom => new_zoom_level_result = ZoomLevel::unzoom(&current_zoom_level)
    }

    let new_zoom_level: ZoomLevel = match new_zoom_level_result {
        Some(v) => v,
        None => panic!()
    };

    tmp_projection.scaling_mode = ScalingMode::FixedVertical(new_zoom_level as i32 as f32);
    **cam_projection = Projection::Orthographic(tmp_projection.clone());

}
