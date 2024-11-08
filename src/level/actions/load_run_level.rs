
use bevy::prelude::*;
use bevy::utils::Uuid;
use crate::level::definition::level_definition::{
    LevelGrid,
    LevelUid,
    ResCurrentLevel,
    ResCurrentLevelGrid
};

pub fn create_new_level(
    commands: &mut Commands,
) -> Uuid {
    let uid = Uuid::new_v4();
    commands.spawn(
        (
            LevelGrid{
                ..Default::default()
            },
            LevelUid{
                uid:Some(uid) // TODO: REMOVE uid
            },
        )
    );
    uid
}

pub fn run_level(
    level_uid : Uuid,
    r_cur_level: &mut ResMut<ResCurrentLevel>,
    r_cur_level_grid: &mut ResMut<ResCurrentLevelGrid>,
    q_levels: &Query<(Entity, &LevelUid, &LevelGrid)>,
) {

    let mut entity:Option<Entity> = None;
    let mut level_grid:Option<LevelGrid> = None;

    let mut is_level_found = false;
    for (i_entity, i_level_uid, i_level_grid) in q_levels.iter() {
        // FIXME: rust syntaxic sugar to do this better.
        if i_level_uid.uid.is_none() {
            continue
        }
        if level_uid != i_level_uid.uid.unwrap() {
            continue 
        }
        entity = Some(i_entity);
        level_grid = Some(*i_level_grid);
        is_level_found = true;
    }

    if !is_level_found {
        return
    }

    r_cur_level_grid.level_grid = level_grid.unwrap().level_grid.clone();
    r_cur_level_grid.hedgehog_grid = level_grid.unwrap().hedgehog_grid.clone();
    r_cur_level.level_uid = Some(level_uid);
    r_cur_level.level_entity = entity;
}

pub fn dump_current_level_ressource_to_components(
    r_cur_level: &mut ResMut<ResCurrentLevel>,
    r_cur_level_grid: &mut ResMut<ResCurrentLevelGrid>,
    q_levels: &mut Query<(Entity, &mut LevelGrid)>,
){
    let level_entity = r_cur_level.level_entity;
    if level_entity.is_none() {
        return;
    }
    for (entity, mut level_grid) in q_levels.iter_mut() {
        if entity != level_entity.unwrap() {
            return;
        }
        level_grid.level_grid = r_cur_level_grid.level_grid.clone();
        level_grid.hedgehog_grid = r_cur_level_grid.hedgehog_grid.clone();
    }
}
