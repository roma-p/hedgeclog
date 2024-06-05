
use bevy::prelude::*;
use hedgeclog::app::builder_bevy_app::{
    add_plugins_default,
    add_plugins_dev,
    set_episode_location,
    AppType

};


fn main() {
    let mut app = App::new();
    add_plugins_default(&mut app);
    add_plugins_dev(&mut app);
    set_episode_location(&mut app, AppType::Construct);

    app.run();
}
