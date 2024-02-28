use bevy::prelude::*;
use bevy::transform::commands;
use crate::config::{StateGlobal, StateEditorLoaded};
use crate::common::level::{BundleTile, BundleTileLevel, LEVEL_ORIGIN, GridPosition};
use crate::common::asset_loader::SceneAssets;

// -- STATE ------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum StateSelectedTile {
    TileFloor,
    TileWater,
    TileFire,
}

// -- BUNDLE : TILES ---------------------------------------------------------

// marker component
#[derive(Component)]
pub struct MarkerTileSelector;

// marker component
#[derive(Component)]
pub struct MarkerTextLoadingEditor;

#[derive(Component)]
pub struct MarkerTileSample;

// tiles used to define BundleTileBuilder type.
#[derive(Bundle)]
pub struct BundleTileSelector{
    pub tile: BundleTile,
    pub tile_type: MarkerTileSelector,
}

// singleton of the tile display to edit level.
#[derive(Bundle)]
pub struct BundleTileSample{
    pub tile: BundleTile,
}

// used to detect mouse cursor position with level editor.
#[derive(Component)]
struct GroundPlane;


// -- COMPONENT : GUI --------------------------------------------------------

#[derive(Component)]
struct MarkerEditorGUI;

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEditor;

impl Plugin for PluginEditor{
    fn build(&self, app: &mut App){
        app.add_systems(
            Update, 
            (
                user_input_editor.run_if(in_state(StateGlobal::Editor)),
                edit_level.run_if(in_state(StateGlobal::Editor)),
                editor_loading_prepare.run_if(
                    in_state(StateGlobal::Editor).and_then(
                    in_state(StateEditorLoaded::NotLoaded))
                ),
                editor_loading_load.run_if(
                    in_state(StateEditorLoaded::Loading)
                ),
            ),
        );
        app.add_systems(
            OnEnter(StateGlobal::Editor),
                editor_setup.run_if(in_state(StateEditorLoaded::Loaded))
        );
        app.add_systems(OnEnter(StateEditorLoaded::Loaded), editor_setup);
        app.add_systems(OnExit(StateGlobal::Editor), editor_teardown);
    }
}


// -- SYSTEM -----------------------------------------------------------------

fn editor_loading_prepare(
    mut commands: Commands,
    mut state_global: ResMut<NextState<StateEditorLoaded>>,
) {
    commands.spawn(
        (
            TextBundle::from_section(
                "Loading editor...",
                TextStyle {
                    font_size: 25.0,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            }),
            MarkerTextLoadingEditor,
        )
    );
    state_global.set(StateEditorLoaded::Loading);
}

fn editor_loading_load(
    mut commands: Commands,
    mut state_global: ResMut<NextState<StateEditorLoaded>>,
    entity: Query<Entity, With <MarkerTextLoadingEditor>>
) {
    // what do we do here...
    // loading the off screen tile selector.
    // Loading the necessary text.
    // We spawn tile selector only when loading and running?
     
    commands.entity(entity.single()).despawn();
    state_global.set(StateEditorLoaded::Loaded);
}


fn editor_setup(
    mut commands: Commands,
) {
    commands.spawn(
        (
            TextBundle::from_section(
                "Level editor.",
                TextStyle {
                    font_size: 25.0,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            }),
            MarkerEditorGUI,
        )
    );
}

fn editor_teardown(
    mut commands: Commands,
    entity: Query<Entity, With <MarkerEditorGUI>>
) {
    commands.entity(entity.single()).despawn();
}


fn edit_level(mut commands: Commands) {
}


fn user_input_editor(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state_global: ResMut<NextState<StateGlobal>>,

) {
    if keyboard_input.pressed(KeyCode::Escape) {
        state_global.set(StateGlobal::Game); 
    }
    // add move hedghog, restart, undo.
} 


fn spawn_selector_entity(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn(BundleTileSelector{
        tile: BundleTile{
            model: SceneBundle {
                scene: scene_assets.tile_floor.clone(),
                transform: Transform::from_translation(LEVEL_ORIGIN),
                ..default()
            }, 
            grid_position: GridPosition{
                value: IVec2::new(0, 0)
            }
        },
        tile_type:MarkerTileSelector
    });
}

fn spawn_level_tile_from_selector(
        mut commands: Commands,
        selector_query: Query<(&GridPosition, &Transform, &Handle<Scene>), With <MarkerTileSelector>>
) {
    let (grid_position, transform, scene_handle) = selector_query.single();
    let level_tile = BundleTileLevel{
        tile : BundleTile {
            model: SceneBundle {
                scene: scene_handle.clone(),
                transform: transform.clone(),
                ..default()
            },
            grid_position: grid_position.clone()
        }
    };
    commands.spawn(level_tile);
}

