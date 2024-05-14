
use bevy::prelude::*;

use crate::config::{StateLevelLoaded, LEVEL_DEFAULT_SIZE, StateUserInputAllowed};
use crate::common::tiles::EnumeTileBehaviour;

pub const LEVEL_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);

pub enum ZOOM_LEVEL {
    REALLYSMALL = 6,
    SMALL = 10,
    NORMAL = 12,
    BIG = 15,
    REALLYBIG = 20,
}

impl ZOOM_LEVEL {
    pub fn unzoom(&self) -> Option<ZOOM_LEVEL> {
        match self {
            ZOOM_LEVEL::REALLYSMALL => Some(ZOOM_LEVEL::SMALL),
            ZOOM_LEVEL::SMALL => Some(ZOOM_LEVEL::NORMAL),
            ZOOM_LEVEL::NORMAL => Some(ZOOM_LEVEL::BIG),
            ZOOM_LEVEL::BIG => Some(ZOOM_LEVEL::REALLYBIG),
            ZOOM_LEVEL::REALLYBIG => Some(ZOOM_LEVEL::REALLYBIG),
            _ => None
        }
    }

    pub fn zoom(&self) -> Option<ZOOM_LEVEL> {
        match self {
            ZOOM_LEVEL::REALLYBIG => Some(ZOOM_LEVEL::BIG),
            ZOOM_LEVEL::BIG => Some(ZOOM_LEVEL::NORMAL),
            ZOOM_LEVEL::NORMAL => Some(ZOOM_LEVEL::SMALL),
            ZOOM_LEVEL::SMALL => Some(ZOOM_LEVEL::REALLYSMALL),
            ZOOM_LEVEL::REALLYSMALL => Some(ZOOM_LEVEL::REALLYSMALL),
            _ => None
        }
    }

    pub fn get_from_i32(value: i32) -> Option<ZOOM_LEVEL> {

        const I32_REALLYSMALL: i32 = ZOOM_LEVEL::REALLYSMALL as i32;
        const I32_SMALL: i32 = ZOOM_LEVEL::SMALL as i32;
        const I32_NORMAL: i32 = ZOOM_LEVEL::NORMAL as i32;
        const I32_BIG: i32 = ZOOM_LEVEL::BIG as i32;
        const I32_REALLYBIG: i32 = ZOOM_LEVEL::REALLYBIG as i32;

        match value {
            I32_REALLYSMALL => Some(ZOOM_LEVEL::REALLYSMALL),
            I32_SMALL => Some(ZOOM_LEVEL::SMALL),
            I32_NORMAL=> Some(ZOOM_LEVEL::NORMAL),
            I32_BIG => Some(ZOOM_LEVEL::BIG),
            I32_REALLYBIG => Some(ZOOM_LEVEL::REALLYBIG),
            _ => None
            // _ => Some(ZOOM_LEVEL::NORMAL)  // DEFAULTING TO ZERO, FIXME: rather panic?
        }
    }
}

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
