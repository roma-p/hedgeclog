
use bevy::prelude::*;
use hedgeclog::app::builder_bevy_app::{
    configure_default,
    configure_dev,
    set_episode_location,
    AppType

};

use hedgeclog::episode::edit_episode::{
    s_create_origin_level_on_curr_episode, 
    s_create_new_episode
};
use hedgeclog::episode::load_run_episode::{
    s_run_only_episode,
    s_run_origin_level_on_curr_episode
};


fn main() {
    let mut app = App::new();

    configure_default(&mut app);
    configure_dev(&mut app);

    set_episode_location(&mut app, AppType::Construct);

    app
        .add_systems(
            PostStartup,
            (
                s_create_new_episode, 
                s_run_only_episode,
                s_create_origin_level_on_curr_episode,
                s_run_origin_level_on_curr_episode
            ).chain()
        )
        .run();

// create_new_episode() OK
// load_only_episode() -> current level, NONE doit être possible, oui car spawn entity. OK
// create_new_level() -> at default start location.
// load_episode_origin() 

// ET VOILA.

}
