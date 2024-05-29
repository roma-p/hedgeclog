use bevy::prelude::*;

use crate::config::*;
use crate::level::definition::camera::*;
use crate::level::definition::hedgehog::*;
use crate::level::definition::level_definition::*;
use crate::level::definition::level_definition::{LevelGrid, LevelGridTile};
use crate::level::definition::tiles::*;

use crate::level::actions::edit_level::*;
use crate::level::actions::serialize::*;

// CONST / ENUM / EVENT / COMPONENT / RESSOURCE ------------------------------

#[derive(Component)]
struct MarkerEditorGUI;

// marker component
#[derive(Component)]
pub struct MarkerTextLoadingLevel;

// TODO (roadmap, no neorg)
// levelInfo (name / id / size) for every level.
// but also dynamic scene for dynmaic scene.
// how do I get the tiles of hedgehog of every level: using parenting (see unoficial cheatbook)
// so when adding a "tile" or a "hedgeclog" or a object to the level:
// add it as a child of a level. (can we have multiple parents?), and to the dynamic scene.
// so no need to for level description. We serialize the entirety of the level. (simpler for custom
// levels actually...)

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginLevel;

impl Plugin for PluginLevel {
    fn build(&self, app: &mut App) {
        app.add_plugins(PluginCamera)
            .add_plugins(PluginHedghog)
            .add_plugins(PluginTiles)
            .add_plugins(PluginLevelDefinition)
            .add_plugins(PluginEditLevel)
            .add_plugins(PluginSerialize)
            .insert_resource(LevelGrid::default())
            .add_systems(
                PostStartup,
                level_loading_prepare.run_if(in_state(StateLevelLoaded::NotLoaded)),
            )
            .add_systems(OnEnter(StateLevelLoaded::Loading), level_loading_load);
    }
}

// -- SYSTEM -----------------------------------------------------------------

// TOOD -> move to actions/load_level
//
// -- loading --

fn level_loading_prepare(
    mut commands: Commands,
    mut state_level_loaded: ResMut<NextState<StateLevelLoaded>>,
) {
    commands.spawn((
        TextBundle::from_section(
            "Loading level...",
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
        MarkerTextLoadingLevel,
    ));
    state_level_loaded.set(StateLevelLoaded::Loading);
}

fn level_loading_load(
    mut commands: Commands,
    mut s_level_loaded: ResMut<NextState<StateLevelLoaded>>,
    mut r_level_grid: ResMut<LevelGrid>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
    entity: Query<Entity, With<MarkerTextLoadingLevel>>,
) {
    // const ARRAY_REPEAT_VALUE:EnumeTileBehaviour = EnumeTileBehaviour::Empty;
    // FIXME: WAIT WILL IT BE THE SAME REFERENCE?
    const ARRAY_REPEAT_VALUE: LevelGridTile = LevelGridTile {
        tile_behaviour: EnumeTileBehaviour::Empty,
        tile_id: None,
        tile_entity: None,
    };
    r_level_grid.level_grid = [[ARRAY_REPEAT_VALUE; LEVEL_DEFAULT_SIZE]; LEVEL_DEFAULT_SIZE];
    commands.entity(entity.single()).despawn();
    s_level_loaded.set(StateLevelLoaded::Loaded);
    s_user_input_allowed.set(StateUserInputAllowed::Allowed);
}
