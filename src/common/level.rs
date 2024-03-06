use bevy::{prelude::*, render::camera::ScalingMode};

use crate::common::asset_loader::SceneAssets;
use crate::config::{StateGlobal, StateLevelLoaded, StateEditorView};
use crate::common::camera::{MarkerCamera, BundleCameraInfo, MarkerCameraInfoDefault};

pub const LEVEL_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);

// -- COMPONENENTS : GRID INFO -----------------------------------------------

#[derive(Component, Debug, Clone)]
pub struct GridPosition {
    pub value: IVec2
}

// -- COMPONENT : GUI --------------------------------------------------------

#[derive(Component)]
struct MarkerEditorGUI;

// marker component
#[derive(Component)]
pub struct MarkerTextLoadingLevel;


// -- BUNDLE : TILES ---------------------------------------------------------

#[derive(Bundle)]
pub struct BundleTile{
    pub model: SceneBundle,
    pub grid_position: GridPosition,
}

// tiles that actually compose the level
#[derive(Bundle)]
pub struct BundleTileLevel{
    pub tile: BundleTile,
}

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginLevel;

impl Plugin for PluginLevel{
    fn build(&self, app: &mut App){
        // app.add_systems(PostStartup, spawn_level_tile_from_selector);
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
    mut state_level_loaded: ResMut<NextState<StateLevelLoaded>>,
    scene_assets: Res<SceneAssets>,
    entity: Query<Entity, With <MarkerTextLoadingLevel>>
) {
    commands.spawn(BundleTile{
        model: SceneBundle {
            scene: scene_assets.tile_floor.clone(),
            transform: Transform::from_translation(LEVEL_ORIGIN),
            ..default()
        }, 
        grid_position: GridPosition{
            value: IVec2::new(0, 0)
        }
    });
    commands.spawn(BundleTile{
        model: SceneBundle {
            scene: scene_assets.tile_fire.clone(),
            transform: Transform::from_translation(
                Vec3::new(2.0, 0.0, 0.0)
                
            ),
            ..default()
        }, 
        grid_position: GridPosition{
            value: IVec2::new(0, 0)
        }
    });
    commands.entity(entity.single()).despawn();
    state_level_loaded.set(StateLevelLoaded::Loaded);
}
