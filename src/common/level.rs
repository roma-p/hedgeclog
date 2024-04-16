use bevy::prelude::*;

use crate::config::{StateLevelLoaded, LEVEL_DEFAULT_SIZE, StateUserInputAllowed};
use crate::common::tiles::EnumeTileBehaviour;

pub const LEVEL_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);


#[derive(Resource, Debug, Default)]
pub struct LevelGrid {
    pub level_grid: [[EnumeTileBehaviour; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
}

#[derive(Component)]
struct MarkerEditorGUI;

// marker component
#[derive(Component)]
pub struct MarkerTextLoadingLevel;

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginLevel;

impl Plugin for PluginLevel{
    fn build(&self, app: &mut App){
        app.insert_resource(LevelGrid::default());
        app.add_systems(
            PostStartup,
            (
                level_loading_prepare.run_if(in_state(StateLevelLoaded::NotLoaded)),
            )
        );
        app.add_systems(OnEnter(StateLevelLoaded::Loading), level_loading_load);
    }
}

// -- SYSTEM -----------------------------------------------------------------

// -- loading ----------------------------------------------------------------

fn level_loading_prepare(
    mut commands: Commands,
    mut state_level_loaded: ResMut<NextState<StateLevelLoaded>>,
) {
    commands.spawn(
        (
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
        )
    );
    state_level_loaded.set(StateLevelLoaded::Loading);
}

fn level_loading_load(
    mut commands: Commands,
    mut s_level_loaded: ResMut<NextState<StateLevelLoaded>>,
    mut r_level_grid: ResMut<LevelGrid>,
    mut s_user_input_allowed: ResMut<NextState<StateUserInputAllowed>>,
    entity: Query<Entity, With <MarkerTextLoadingLevel>>
) {
    const ARRAY_REPEAT_VALUE:EnumeTileBehaviour = EnumeTileBehaviour::Empty;
    r_level_grid.level_grid = [[ARRAY_REPEAT_VALUE; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE];
    commands.entity(entity.single()).despawn();
    s_level_loaded.set(StateLevelLoaded::Loaded);
    s_user_input_allowed.set(StateUserInputAllowed::Allowed);
}
