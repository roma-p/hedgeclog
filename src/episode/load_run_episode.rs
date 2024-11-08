use bevy::prelude::*;
use bevy::utils::Uuid;
use crate::episode::episode_definition::*;
use crate::level::definition::level_definition::*;
use crate::level::actions::load_run_level::run_level;

pub fn s_run_only_episode(
    q_episodes: Query<(Entity, &EpisodeUID, &EpisodeGrid)>,
    mut r_cur_episode: ResMut<ResCurEpisode>,
    mut r_cur_episode_grid: ResMut<ResCurEpisodeGrid>,
) {
    
    // will panic if more than one episode.
    let (entity, episode_uid, episode_grid) = q_episodes.single();
    run_episode(
        &mut r_cur_episode,
        &mut r_cur_episode_grid,
        &entity,
        &episode_uid,
        &episode_grid
    );
}

pub fn s_run_origin_level_on_curr_episode(
    r_cur_episode_grid: Res<ResCurEpisodeGrid>,
    mut r_cur_level: ResMut<ResCurrentLevel>,
    mut r_cur_level_grid: ResMut<ResCurrentLevelGrid>,
    q_levels: Query<(Entity, &LevelUid, &LevelGrid)>,
) {

    let x = DEFAULT_EPISODE_START_LOCATION[0];
    let y = DEFAULT_EPISODE_START_LOCATION[1];

    let level_uid_option = r_cur_episode_grid.episode_grid[x][y];
    let level_uid: Uuid;

    // FIXME: rust syntaxic sugar to do this better.
    if level_uid_option.is_some() {
        level_uid = level_uid_option.unwrap();
    } else {
        return
    }
    run_level(level_uid, &mut r_cur_level, &mut r_cur_level_grid, &q_levels);
}

// -- FUNCS ------------------------------------------------------------------


pub fn run_episode(
    r_cur_episode: &mut ResMut<ResCurEpisode>,
    r_cur_episode_grid: &mut ResMut<ResCurEpisodeGrid>,
    episode_entity: &Entity,
    episode_uid: &EpisodeUID,
    episode_grid: &EpisodeGrid,
    // missing: a query with all levels!

) {
    r_cur_episode.episode_uid = Some(episode_uid.uid);
    r_cur_episode.episode_entity = Some(*episode_entity);
    r_cur_episode_grid.episode_grid = episode_grid.episode_grid.clone();
    // TODO: find all levels and load them !!
    // if none, skip!
}

