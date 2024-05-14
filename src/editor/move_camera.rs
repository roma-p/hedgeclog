use bevy::prelude::*;
use crate::common::camera::{
    MarkerCamera,
    translate_camera,
    zoom_camera, ZoomCameraMode
};
use crate::config::StateGlobal;
use crate::common::tiles::TILE_SIZE;


// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditorCameraMovement;

// TODO: set input mode tile... ???

impl Plugin for PluginEditorCameraMovement{
    fn build(&self, app: &mut App){
        app
            .add_systems(
                Update,
                pan_camera 
                .run_if(in_state(StateGlobal::EditorRunning))
            );
    }
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
