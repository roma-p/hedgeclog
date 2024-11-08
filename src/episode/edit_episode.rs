use bevy::prelude::*;
use bevy::utils::Uuid;
use crate::episode::episode_definition::*;
use crate::level::actions::load_run_level::create_new_level;
use crate::app::common::*;


pub fn s_create_new_episode(
    mut commands: Commands,
    mut r_current_episode: ResMut<ResCurEpisode>,
    r_episode_location: Res<ResEpisodeLocation>,
) {
    create_new_episode(
        &mut commands,
        &r_episode_location,
        "construct",
        "construct"
    );
}

pub fn s_create_origin_level_on_curr_episode(
    mut commands: Commands,
    mut r_cur_episode_grid: ResMut<ResCurEpisodeGrid>,
) {
    let x = DEFAULT_EPISODE_START_LOCATION[0];
    let y = DEFAULT_EPISODE_START_LOCATION[1];
    create_level_on_curr_episode_at_position(
        x, y, 
        &mut commands,
        &mut r_cur_episode_grid
    );
}

fn create_new_episode(
    commands: &mut Commands,
    r_episode_location: &Res<ResEpisodeLocation>,
    episode_name: &str, 
    episode_filename: &str,
    // TODO: parents episode position?? not useful for construct...
) -> Uuid {


    let uid = Uuid::new_v4();

    let episode_grid = [
        [None; EPISODE_DEFAULT_HORIZONTAL_SIZE];
        EPISODE_DEFAULT_VERTICAL_SIZE
    ];

    let entity_command = commands.spawn(
        (
            // TODO -> make a bandle out of this.
            EpisodeInfo{
                level_name: episode_name.to_string(),
                // level_path: "assets/level_desc/truc.ron".to_string()
                level_path: format!(
                    "assets/{:?}/{:?}.ron",
                    r_episode_location.episode_location,
                    episode_filename
                )
            },
            EpisodeUID{ uid: Uuid::new_v4() },
            EpisodeGrid{ episode_grid }
        )
    );

    uid
}

fn create_level_on_curr_episode_at_position(
    pos_x: usize,
    pos_y: usize,
    commands: &mut Commands,
    r_cur_episode_grid: &mut ResMut<ResCurEpisodeGrid>,
){
    let level_uid = create_new_level(commands);
    r_cur_episode_grid.episode_grid[pos_x][pos_y] = Some(level_uid);
    //TODO: add checks on 'pos_x' 'pos_y' -> level size...
}

