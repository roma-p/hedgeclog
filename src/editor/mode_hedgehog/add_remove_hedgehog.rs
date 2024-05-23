use bevy::prelude::*;

use crate::config::{StateGlobal, StateUserInputAllowed};
use crate::level::definition::level_definition::GridPosition;
use crate::level::definition::hedgehog::{
    BundleHedgehog,
    ResHedgeHogInfo,
    EnumHedgehogOnGrid,
    HedgehogType,
};
use crate::asset::asset_loader::HedgehogAssets;
use crate::editor::common::{
    EventEditorSubSystemSetup,
    StateEditorLoaded,
    EventCursorGridPositionChanged,
    StateEditorMode,
    SSetEditor,
};
use crate::level::definition::level_definition::{
    LevelGrid, LEVEL_ORIGIN,
};
use crate::level::actions::edit_level::{
    EventHedgehogCreationAsked,
    EventHedgehogRemovalAsked
};
use crate::editor::cursor_to_world::CursorGridPosition;

use crate::level::definition::tiles::{EnumeTileBehaviour, TILE_SIZE};

// -- COMPONENTS / RESSOURCES STATES -----------------------------------------

#[derive(Component)]
pub struct MarkerHedgehogCreator;

#[derive(Event)]
pub struct EventHedgehogCreated;

#[derive(Event)]
pub struct EventHedgehogRemoved;

#[derive(Resource, Debug, Default)]
struct ModeHedgehogLocalBuffer {
    pub hover_hedgehog_grid_position: GridPosition
}

pub struct PluginAddRemoveHedgehog;

// -- PLUGIN -----------------------------------------------------------------

impl Plugin for PluginAddRemoveHedgehog{
    fn build(&self, app: &mut App){
        app
            .add_event::<EventHedgehogCreated>()
            .add_event::<EventHedgehogRemoved>()
            .insert_resource(ModeHedgehogLocalBuffer::default())
            .add_systems(OnEnter(StateEditorLoaded::LoadedAndSetuping), setup)
            .add_systems(OnExit(StateGlobal::EditorRunning), teardown)
            .add_systems(
                Update,
                (
                    user_input
                        .in_set(SSetEditor::UserInput)
                        .run_if(in_state(StateEditorMode::Hedgehog)),
                    update_hedgehog_creator_position
                        .run_if(on_event::<EventCursorGridPositionChanged>()
                        .and_then(in_state(StateEditorMode::Hedgehog))),
                    create_hedgehog
                        .run_if(on_event::<EventHedgehogCreated>()),
                    remove_hedgehog
                        .run_if(on_event::<EventHedgehogRemoved>()),
                )
            );
    }
}

// -- SYSTEMS ----------------------------------------------------------------

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    r_hedgehog: Res<HedgehogAssets>,
    r_hedgehog_info: Res<ResHedgeHogInfo>,
    mut e_editor_subsystem_setup: EventWriter<EventEditorSubSystemSetup>,
) {
    let hedgehog_material = materials.add(
        StandardMaterial{
            base_color_texture: Some(r_hedgehog.sprite_idle.clone()),
            base_color: Color::rgba(1.0, 1.0, 1.0, 0.8),
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        }
    );
    commands.spawn(
        (
            BundleHedgehog {
                model: PbrBundle {
                    mesh: meshes.add(Mesh::from(Plane3d{normal: Direction3d::Y})),
                    material: hedgehog_material,
                    visibility: Visibility::Hidden,
                    transform: r_hedgehog_info.transform_shift.clone(),
                    ..Default::default()
                },
                grid_position: GridPosition {
                    x : 0,
                    z : 0,
                },
                hedgehog_type: HedgehogType::HedegehogeTypeStandard,
            }, 
            MarkerHedgehogCreator,
        )
    );
    e_editor_subsystem_setup.send(EventEditorSubSystemSetup);
}

fn teardown(
    mut commands: Commands,
    q_hedgehog_creator: Query<Entity, With <MarkerHedgehogCreator>>,
) {
    commands.entity(q_hedgehog_creator.single()).despawn_recursive();
}

fn user_input(
    r_mouse_input: Res<ButtonInput<MouseButton>>,
    mut e_h_created: EventWriter<EventHedgehogCreated>,
    mut e_h_removed: EventWriter<EventHedgehogRemoved>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
) {
    if r_mouse_input.just_pressed(MouseButton::Left) {
        s_user_input_allowed.set(StateUserInputAllowed::NotAllowed);  // -> set to Allowed by add_remove_tile.create_tile
        e_h_created.send(EventHedgehogCreated);
    } else if r_mouse_input.just_pressed(MouseButton::Right) {
        s_user_input_allowed.set(StateUserInputAllowed::NotAllowed);  // -> set to Allowed by add_remove_tile.remove_tile
        e_h_removed.send(EventHedgehogRemoved);
    }
} 

// TODO: globlal logic for userinput allowed.

fn update_hedgehog_creator_position(
    r_cursor_grid_position: Res<CursorGridPosition>,
    mut r_hedgehog_builder_info: ResMut<ModeHedgehogLocalBuffer>,
    r_hedgehog_info: Res<ResHedgeHogInfo>,
    mut q_hedgehog_creator: Query<&mut Transform, With <MarkerHedgehogCreator>>,
    r_grid : Res<LevelGrid>,
) {
    let grid_pos_x = r_cursor_grid_position.grid_pos_x;
    let grid_pos_z = r_cursor_grid_position.grid_pos_z;

    // hedgehog can only be created on floors.
    match r_grid.level_grid[grid_pos_x][grid_pos_z].tile_behaviour {
        EnumeTileBehaviour::TileBFloor => {},
        _ => return
    }

    // hedgehog can only be created if there is no hedgehog on that tile.
    match r_grid.hedgehog_grid[grid_pos_x][grid_pos_z].hedgehog_behaviour {
        EnumHedgehogOnGrid::Empty => {},
        _ => return
    }

    // registering grid position.
    r_hedgehog_builder_info.hover_hedgehog_grid_position = GridPosition{
        x: grid_pos_x,
        z: grid_pos_z,
    };

    let mut transform = q_hedgehog_creator.single_mut();
    *transform = Transform::from_translation(
        LEVEL_ORIGIN + Vec3::new(
                TILE_SIZE * grid_pos_x as f32,
                0.0,
                TILE_SIZE * grid_pos_z as f32,
        )
    );
    *transform = transform.mul_transform(r_hedgehog_info.transform_shift);
}

fn create_hedgehog(
    q_hedgehog_creator: Query<&mut Transform, With <MarkerHedgehogCreator>>,
    r_hedgehog_builder_info: ResMut<ModeHedgehogLocalBuffer>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
    mut e_event_hedgehog_creation_asked: EventWriter<EventHedgehogCreationAsked>,
) {
    e_event_hedgehog_creation_asked.send(
        EventHedgehogCreationAsked{
            hedgehog_transform: q_hedgehog_creator.single().clone(),
            grid_position: r_hedgehog_builder_info.hover_hedgehog_grid_position.clone(),
        }
    );
    s_user_input_allowed.set(StateUserInputAllowed::Allowed);
}

// When deleting tiles, also delete hedgehog.

fn remove_hedgehog(
    r_cursor_grid_position: Res<CursorGridPosition>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
    mut e_event_hedgehog_removal_asked: EventWriter<EventHedgehogRemovalAsked>,
) {
    e_event_hedgehog_removal_asked.send(
        EventHedgehogRemovalAsked{
            grid_position: GridPosition{
                x: r_cursor_grid_position.grid_pos_x,
                z: r_cursor_grid_position.grid_pos_z,
            }
        }
    );
    s_user_input_allowed.set(StateUserInputAllowed::Allowed);
}
