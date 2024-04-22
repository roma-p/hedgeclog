use bevy::prelude::*;
use crate::common::camera::{MarkerCamera, MarkerCameraInfoDefault};
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
         &mut Transform,
        (With <MarkerCamera>, Without<MarkerCameraInfoDefault>)
    >,
){

    let mut cam_transform = camera_query.single_mut();

    if r_keyboard_input.just_pressed(KeyCode::KeyH) || r_keyboard_input.just_pressed(KeyCode::ArrowLeft) {
       cam_transform.translation = cam_transform.translation.mul_add(
            Vec3::ONE,
            Vec3{
                x: TILE_SIZE,
                y: 0.0,
                z: -TILE_SIZE
            }
        );
    }
    if r_keyboard_input.just_pressed(KeyCode::KeyL) || r_keyboard_input.just_pressed(KeyCode::ArrowRight) {
       cam_transform.translation = cam_transform.translation.mul_add(
            Vec3::ONE,
            Vec3{
                x: -TILE_SIZE,
                y: 0.0,
                z: TILE_SIZE
            }
        );
    }
    if r_keyboard_input.just_pressed(KeyCode::KeyK) || r_keyboard_input.just_pressed(KeyCode::ArrowUp) {
       cam_transform.translation = cam_transform.translation.mul_add(
            Vec3::ONE,
            Vec3{
                x: -TILE_SIZE,
                y: 0.0,
                z: -TILE_SIZE
            }
        );
    }
    if r_keyboard_input.just_pressed(KeyCode::KeyJ) || r_keyboard_input.just_pressed(KeyCode::ArrowDown) {
       cam_transform.translation = cam_transform.translation.mul_add(
            Vec3::ONE,
            Vec3{
                x: TILE_SIZE,
                y: 0.0,
                z: TILE_SIZE
            }
        );
    }

}
