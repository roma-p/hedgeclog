use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::editor::common::{EventEditorSubSystemLoaded, StateEditorLoaded};
use crate::common::camera::MarkerCamera;
use crate::config::StateGlobal;

// -- COMPONENT --------------------------------------------------------------

#[derive(Resource, Default)]
pub struct CursorToGroundCoordonate {
    pub global: Vec3,
    pub local: Vec2,
}

#[derive(Component)]
struct MarkerGroundPlane;

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginCursorToWorld;

impl Plugin for PluginCursorToWorld{
    fn build(&self, app: &mut App){
        app
            .insert_resource(CursorToGroundCoordonate::default())
            .add_systems(OnEnter(StateEditorLoaded::Loading) , load)
            .add_systems(Update,
                process.run_if(
                    in_state(StateGlobal::Editor).and_then(
                    in_state(StateEditorLoaded::Loaded))
                )
            );
         
    }
}

// -- SYSTEM -----------------------------------------------------------------

pub fn load(
    mut commands: Commands,
    mut e_editor_subsystem_loaded: EventWriter<EventEditorSubSystemLoaded>,
) {
    commands.spawn((
        MarkerGroundPlane,
        PbrBundle {
            transform: Transform::default(),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));
    e_editor_subsystem_loaded.send(EventEditorSubSystemLoaded);
}

fn dispose(
    mut commands: Commands,
    q_plane: Query<Entity, With<MarkerGroundPlane>>,
) {
    commands.entity(q_plane.single()).despawn();
}


// TODO: do i really need the plane (that can be moved...) 
// or do i only need the inversed matrix?
fn process(
    mut cursor_to_ground_coord: ResMut<CursorToGroundCoordonate>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MarkerCamera>>,
    q_plane: Query<&GlobalTransform, With<MarkerGroundPlane>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let ground_transform = q_plane.single();
    let window = q_window.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let plane_origin = ground_transform.translation();
    let plane_normal = ground_transform.up();

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // TODO: new this Plane at loading.
    let Some(distance) = ray.intersect_plane(plane_origin, Plane3d::new(plane_normal)) else {
        return;
    };

    let global_cursor = ray.get_point(distance);
    cursor_to_ground_coord.global = global_cursor;

    let inverse_transform_matrix = ground_transform.compute_matrix().inverse();
    let local_cursor = inverse_transform_matrix.transform_point3(global_cursor);
    cursor_to_ground_coord.local = local_cursor.xz();
}
