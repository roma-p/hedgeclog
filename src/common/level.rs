
use bevy::prelude::*;

use crate::config::{StateLevelLoaded, LEVEL_DEFAULT_SIZE, StateUserInputAllowed};
use crate::common::tiles::EnumeTileBehaviour;
use crate::common::hedgehog::EnumHedgehogOnGrid;

pub const LEVEL_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);

pub enum ZoomLevel {
    REALLYSMALL = 6,
    SMALL = 10,
    NORMAL = 12,
    BIG = 15,
    REALLYBIG = 20,
}

impl ZoomLevel {
    pub fn unzoom(&self) -> Option<ZoomLevel> {
        match self {
            ZoomLevel::REALLYSMALL => Some(ZoomLevel::SMALL),
            ZoomLevel::SMALL => Some(ZoomLevel::NORMAL),
            ZoomLevel::NORMAL => Some(ZoomLevel::BIG),
            ZoomLevel::BIG => Some(ZoomLevel::REALLYBIG),
            ZoomLevel::REALLYBIG => Some(ZoomLevel::REALLYBIG),
            _ => None
        }
    }

    pub fn zoom(&self) -> Option<ZoomLevel> {
        match self {
            ZoomLevel::REALLYBIG => Some(ZoomLevel::BIG),
            ZoomLevel::BIG => Some(ZoomLevel::NORMAL),
            ZoomLevel::NORMAL => Some(ZoomLevel::SMALL),
            ZoomLevel::SMALL => Some(ZoomLevel::REALLYSMALL),
            ZoomLevel::REALLYSMALL => Some(ZoomLevel::REALLYSMALL),
            _ => None
        }
    }

    pub fn get_from_i32(value: i32) -> Option<ZoomLevel> {

        const I32_REALLYSMALL: i32 = ZoomLevel::REALLYSMALL as i32;
        const I32_SMALL: i32 = ZoomLevel::SMALL as i32;
        const I32_NORMAL: i32 = ZoomLevel::NORMAL as i32;
        const I32_BIG: i32 = ZoomLevel::BIG as i32;
        const I32_REALLYBIG: i32 = ZoomLevel::REALLYBIG as i32;

        match value {
            I32_REALLYSMALL => Some(ZoomLevel::REALLYSMALL),
            I32_SMALL => Some(ZoomLevel::SMALL),
            I32_NORMAL=> Some(ZoomLevel::NORMAL),
            I32_BIG => Some(ZoomLevel::BIG),
            I32_REALLYBIG => Some(ZoomLevel::REALLYBIG),
            _ => None
            // _ => Some(ZOOM_LEVEL::NORMAL)  // DEFAULTING TO ZERO, FIXME: rather panic?
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct LevelGrid {
    pub level_grid: [[EnumeTileBehaviour; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
    pub hedgehog_grid: [[EnumHedgehogOnGrid; LEVEL_DEFAULT_SIZE];LEVEL_DEFAULT_SIZE],
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
