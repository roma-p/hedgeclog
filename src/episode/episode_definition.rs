use bevy::prelude::*;
use bevy::utils::Uuid;

// -- CONSTS -----------------------------------------------------------------

pub const EPISODE_DEFAULT_HORIZONTAL_SIZE: usize = 10;
pub const EPISODE_DEFAULT_VERTICAL_SIZE: usize = 10;

pub const DEFAULT_EPISODE_START_LOCATION: [usize; 2] = [0, 5];

// -- COMPONENTS -------------------------------------------------------------

#[derive(Component, Default, Debug, Clone)]
pub struct EpisodeInfo{
    pub level_name: String,
    pub level_path: String,
}

#[derive(Component, Default, Debug, Clone)]
pub struct EpisodeUID{
    pub uid: Uuid,
}

#[derive(Component, Default, Debug, Clone)]
pub struct EpisodeGrid{
    pub episode_grid: [
        [Option<Uuid>; EPISODE_DEFAULT_HORIZONTAL_SIZE];
        EPISODE_DEFAULT_VERTICAL_SIZE
    ],
}

#[derive(Component, Default, Debug, Clone)]
pub struct EpisodeDescription{
    pub episode_grid: [
        [Option<Uuid>; EPISODE_DEFAULT_HORIZONTAL_SIZE];
        EPISODE_DEFAULT_VERTICAL_SIZE
    ],
}

// -- RESSOURCES -------------------------------------------------------------

#[derive(Resource, Default)]
pub struct ResCurEpisode{
    pub episode_entity: Option<Entity>,
    pub episode_uid: Option<Uuid>,
}

#[derive(Resource, Default)]
pub struct ResCurEpisodeGrid{
    pub episode_grid: [
        [Option<Uuid>; EPISODE_DEFAULT_HORIZONTAL_SIZE];
        EPISODE_DEFAULT_VERTICAL_SIZE
    ],
}

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEpsiodeDefinition;

impl Plugin for PluginEpsiodeDefinition {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ResCurEpisode>()
            .init_resource::<ResCurEpisodeGrid>();
    }
}

