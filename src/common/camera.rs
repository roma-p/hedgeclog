
use bevy::{prelude::*, render::camera::ScalingMode};
use crate::config::{
    StateGlobal,
    StateLevelLoaded,
    // StateEditorView,
    TRANSLATION_LEVEL_ORIGIN,
    // TRANSLATION_EDITOR_TILE_SELECTOR_ORIGIN,
    TRANSLATION_DEFAULT_CAMERA_SHIFT
};

use crate::editor::common::StateEditorView;
use crate::editor::common::TRANSLATION_EDITOR_TILE_SELECTOR_ORIGIN;

// tiles that actually compose the level
#[derive(Bundle, Default)]
pub struct BundleCameraInfo{
    pub projection:OrthographicProjection,
    pub transform: Transform,
}

#[derive(Component)]
pub struct MarkerCamera;

#[derive(Component)]
pub struct MarkerCameraInfoDefault;

#[derive(Component)]
pub struct MarkerCameraInfoEditorTileSelectorView;

pub struct PluginCamera;

impl Plugin for PluginCamera {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_camera);
        app.add_systems(OnEnter(StateLevelLoaded::Loaded), camera_snap_position_default);
        app.add_systems(OnEnter(StateGlobal::Game), camera_snap_position_default.run_if(
            in_state(StateLevelLoaded::Loaded)));

        app.add_systems(OnEnter(StateEditorView::Game), camera_snap_position_default);
        app.add_systems(OnEnter(StateEditorView::TileSelector), camera_snap_position_editor_tile_selector_view);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        (
            Camera3dBundle{
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::FixedVertical(15.0),
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
                    scaling_mode: ScalingMode::FixedVertical(15.0),
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
    commands.spawn(
        (
            BundleCameraInfo{
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::FixedVertical(15.0),
                    ..default()
                }.into(),
                transform: Transform::from_translation(TRANSLATION_EDITOR_TILE_SELECTOR_ORIGIN
                    .mul_add(Vec3::ONE, TRANSLATION_DEFAULT_CAMERA_SHIFT))
                    .looking_at(TRANSLATION_EDITOR_TILE_SELECTOR_ORIGIN, Vec3::Y),
                ..default()
            },
            MarkerCameraInfoEditorTileSelectorView
        )
            
    );
}

fn camera_snap_position_default(
    camera_info_query: Query<
        (&OrthographicProjection, &Transform),
        (With <MarkerCameraInfoDefault>, Without<MarkerCamera>)
    >,
    mut camera_query: Query<
        (&mut Projection, &mut Transform),
        (With <MarkerCamera>, Without<MarkerCameraInfoDefault>)
    >,
) {
    let (src_transform, src_projection) = camera_info_query.single();
    let (mut dst_transform, mut dst_projection) = camera_query.single_mut();
    *dst_transform = Projection::Orthographic(src_transform.clone());
    *dst_projection = src_projection.clone();
}

fn camera_snap_position_editor_tile_selector_view(
    camera_info_query: Query<
        ( &OrthographicProjection, &Transform),
        (With <MarkerCameraInfoEditorTileSelectorView>, Without<MarkerCamera>)
    >,
    mut camera_query: Query<
        (&mut Projection, &mut Transform),
        (With <MarkerCamera>, Without<MarkerCameraInfoEditorTileSelectorView>)
    >,
) {
    let (src_transform, src_projection) = camera_info_query.single();
    let (mut dst_transform, mut dst_projection) = camera_query.single_mut();
    *dst_transform = Projection::Orthographic(src_transform.clone());
    *dst_projection = src_projection.clone();
}
