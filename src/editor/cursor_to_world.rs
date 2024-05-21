use std::{usize, cmp};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::common::level::LEVEL_ORIGIN;
use crate::editor::common::{
    EventCursorGridPositionChanged,
    EventEditorSubSystemLoaded,
    EventEditorSubSystemSetup,
    StateEditorLoaded,
};
use crate::common::camera::MarkerCamera;
use crate::config::StateGlobal;
use crate::common::tiles::TILE_SIZE;

// -- COMPONENT --------------------------------------------------------------

#[derive(Resource, Default)]
pub struct CursorToGroundCoordonate {
    pub global: Vec3,
    pub local: Vec2,
}

#[derive(Resource, Debug, Default)]
pub struct CursorGridPosition {
    pub grid_pos_x: usize,
    pub grid_pos_z: usize,
}

#[derive(Component)]
struct MarkerGroundPlane;

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginCursorToWorld;

impl Plugin for PluginCursorToWorld{
    fn build(&self, app: &mut App){
        app
            .insert_resource(CursorToGroundCoordonate::default())
            .insert_resource(CursorGridPosition::default())
            .add_systems(OnEnter(StateEditorLoaded::Loading), load)
            .add_systems(OnEnter(StateEditorLoaded::LoadedAndSetuping), setup)
            .add_systems(
                Update,
                (
                    update_cursor_to_world
                        .run_if(in_state(StateGlobal::EditorRunning)),
                    update_cursor_to_grid_position
                        .run_if(in_state(StateGlobal::EditorRunning))
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

// -> TODO USEME
// fn dispose(
//     mut commands: Commands,
//     q_plane: Query<Entity, With<MarkerGroundPlane>>,
// ) {
//     commands.entity(q_plane.single()).despawn();
// }

fn setup(
    _commands: Commands,
    mut r_cursor_grid_position: ResMut<CursorGridPosition>,
    mut e_editor_subsystem_setup: EventWriter<EventEditorSubSystemSetup>,
) {
    r_cursor_grid_position.grid_pos_x = 0;
    r_cursor_grid_position.grid_pos_z = 0;
    e_editor_subsystem_setup.send(EventEditorSubSystemSetup);
}

fn update_cursor_to_world(
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

fn update_cursor_to_grid_position(
    mut r_cursor_grid_position: ResMut<CursorGridPosition>,
    r_cursor_to_ground_coordonate: Res<CursorToGroundCoordonate>,
    mut e_cursor_grid_position_changed: EventWriter<EventCursorGridPositionChanged>,
) {
    let previous_grid_pos_x = r_cursor_grid_position.grid_pos_x;
    let previous_grid_pos_z = r_cursor_grid_position.grid_pos_z;

    let local_position = r_cursor_to_ground_coordonate.global - LEVEL_ORIGIN;

    const LEVEL_SIZE:usize = 8;

    let grid_pos_x = cmp::max(
        cmp::min(
            ((local_position.x + (TILE_SIZE / 2.0)) / TILE_SIZE) as usize,
            LEVEL_SIZE - 1
        ),
        0
    );

    let grid_pos_z = cmp::max(
        cmp::min(
            ((local_position.z + (TILE_SIZE / 2.0)) / TILE_SIZE) as usize,
            LEVEL_SIZE - 1
        ),
        0
    );

    r_cursor_grid_position.grid_pos_x = grid_pos_x;
    r_cursor_grid_position.grid_pos_z = grid_pos_z;

    if previous_grid_pos_x != grid_pos_x || previous_grid_pos_z != grid_pos_z {
        e_cursor_grid_position_changed.send(EventCursorGridPositionChanged);
    }
}

