
use bevy::prelude::*;
use bevy::utils::Uuid;

// -- CONSTS -----------------------------------------------------------------

pub const EPISODE_DEFAULT_HORIZONTAL_SIZE: usize = 10;
pub const EPISODE_DEFAULT_VERTICAL_SIZE: usize = 10;

pub const DEFAULT_EPISODE_START_LOCATION: [usize; 2] = [0, 5];

// -- COMPONENTS -------------------------------------------------------------

#[derive(Resource, Default)]
pub struct RessourceCurrentEpisode{
    pub episode_entity: Option<Entity>,
    pub episode_uid: Option<Uuid>,
}

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


// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEpsiode;

impl Plugin for PluginEpsiode {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RessourceCurrentEpisode>()
            .add_systems(PostStartup, create_new_episode);  // TODO: temp...
    }
}

// -- SYSTEM -----------------------------------------------------------------

fn create_new_episode(
    mut commands: Commands,
    mut r_current_episode: ResMut<RessourceCurrentEpisode>,
) {

    let uid = Uuid::new_v4();

    let episode_grid = [
        [None; EPISODE_DEFAULT_HORIZONTAL_SIZE];
        EPISODE_DEFAULT_VERTICAL_SIZE
    ];

    let entity_command = commands.spawn(
        (
            EpisodeInfo{
                level_name: "truc".to_string(),
                level_path: "assets/level_desc/truc.ron".to_string()

            },
            EpisodeUID{ uid: Uuid::new_v4() },
            EpisodeGrid{ episode_grid }
        )
    );

    *r_current_episode = RessourceCurrentEpisode{
        episode_entity: Some(entity_command.id()),
        episode_uid: Some(uid)
    };
}

// -- FUNCS ------------------------------------------------------------------

fn fn_create_new_episode(
    commands: &mut Commands,
    r_current_episode: &mut ResMut<RessourceCurrentEpisode>,
    episode_name: &str, 
    episode_filename: &str,
    // TODO: parents episode position??
) {

    let uid = Uuid::new_v4();

    let episode_grid = [
        [None; EPISODE_DEFAULT_HORIZONTAL_SIZE];
        EPISODE_DEFAULT_VERTICAL_SIZE
    ];

    let entity_command = commands.spawn(
        (
            EpisodeInfo{
                level_name: episode_name.to_string(),
                level_path: "assets/level_desc/truc.ron".to_string()

            },
            EpisodeUID{ uid: Uuid::new_v4() },
            EpisodeGrid{ episode_grid }
        )
    );

    **r_current_episode = RessourceCurrentEpisode{
        episode_entity: Some(entity_command.id()),
        episode_uid: Some(uid)
    };
}

