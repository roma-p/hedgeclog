use bevy::{prelude::*, tasks::IoTaskPool};

use crate::app::builder_bevy_app::ResTypeRegister;
use crate::level::definition::level_definition::{
    LevelGrid, LevelUid, ResCurrentLevelGrid, ResCurrentLevel
};

use crate::episode::episode_definition::{EpisodeGrid, ResCurEpisode};
use crate::level::actions::load_run_level::dump_current_level_ressource_to_components;


fn s_update_current_episode_description(
    mut r_cur_level: ResMut<ResCurrentLevel>,
    mut r_cur_level_grid: ResMut<ResCurrentLevelGrid>,
    r_type_register: Res<ResTypeRegister>,
    r_cur_episode_grid: Res<ResCurEpisode>,
    mut q_levels: Query<(Entity, &mut LevelGrid)>
){
    // TODO: any way to serialize query to pass it to subfunction?
    dump_current_level_ressource_to_components(
        &mut r_cur_level,
        &mut r_cur_level_grid,
        &mut q_levels
    );



}

fn s_dump_episode_description_to_file(){
}
