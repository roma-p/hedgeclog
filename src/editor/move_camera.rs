use bevy::{prelude::*, render::camera::ScalingMode};

use crate::level::definition::level_definition::TRANSLATION_DEFAULT_CAMERA_SHIFT;

use crate::level::definition::camera::{
    MarkerCamera,
    BundleCameraInfo,
    translate_camera,
    zoom_camera,
    ZoomCameraMode, 
    EventCameraSnap,
    camera_snap_position_default
};

use crate::editor::common::{
    EventEditorSubSystemLoaded, StateEditorLoaded,
    TRANSLATION_EDITOR_TILE_SELECTOR_ORIGIN, StateEditorView
};

use crate::config::StateGlobal;

use crate::level::definition::tiles::TILE_SIZE;

#[derive(Component)]
pub struct MarkerCameraInfoEditorTileSelectorView;

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditorCameraMovement;

// TODO: set input mode tile... ???

impl Plugin for PluginEditorCameraMovement{
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(StateEditorLoaded::Loading) , load)
            .add_systems(
                OnEnter(StateEditorView::Level),
                camera_snap_position_default
            )
            .add_systems(
                Update,
                pan_camera 
                .run_if(in_state(StateGlobal::EditorRunning))
            )
            .add_systems(
                OnEnter(StateEditorView::TileSelector),
                camera_snap_position_editor_tile_selector_view
            );
    }
}

pub fn load(
    mut commands: Commands,
    mut e_editor_subsystem_loaded: EventWriter<EventEditorSubSystemLoaded>,
) {
    commands.spawn(
        (
            BundleCameraInfo{
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::FixedVertical(12.0),
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
    e_editor_subsystem_loaded.send(EventEditorSubSystemLoaded);
}

fn pan_camera(
    r_keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<
        (&mut Projection, &mut Transform),
        With <MarkerCamera>
    >,
){

    let (mut cam_projection, mut cam_transform) = camera_query.single_mut();

    if r_keyboard_input.just_pressed(KeyCode::KeyH) || r_keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        translate_camera(
            &mut cam_transform,
            Vec3{ x: -TILE_SIZE, y: 0.0, z: TILE_SIZE }
        );
    }
    else if r_keyboard_input.just_pressed(KeyCode::KeyL) || r_keyboard_input.just_pressed(KeyCode::ArrowRight) {
        translate_camera(
            &mut cam_transform,
            Vec3{ x: TILE_SIZE, y: 0.0, z: -TILE_SIZE }
        );
    }
    else if r_keyboard_input.just_pressed(KeyCode::KeyK) || r_keyboard_input.just_pressed(KeyCode::ArrowUp) {
        translate_camera(
            &mut cam_transform,
            Vec3{ x: -TILE_SIZE, y: 0.0, z: -TILE_SIZE }
        );
    }
    else if r_keyboard_input.just_pressed(KeyCode::KeyJ) || r_keyboard_input.just_pressed(KeyCode::ArrowDown) {
        translate_camera(
            &mut cam_transform,
            Vec3{ x: TILE_SIZE, y: 0.0, z: TILE_SIZE }
        );
    }
    else if r_keyboard_input.just_pressed(KeyCode::KeyB) {
        zoom_camera(&mut cam_projection, ZoomCameraMode::Zoom);
    }
    else if r_keyboard_input.just_pressed(KeyCode::KeyN) {
        zoom_camera(&mut cam_projection, ZoomCameraMode::Unzoom);
    }

}

fn camera_snap_position_editor_tile_selector_view(
    camera_info_query: Query<
        (&Transform, &OrthographicProjection),
        (With <MarkerCameraInfoEditorTileSelectorView>, Without<MarkerCamera>)
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

