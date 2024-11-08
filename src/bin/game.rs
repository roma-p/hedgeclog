use bevy::prelude::*;
use hedgeclog::app::builder_bevy_app::{
    configure_default,
    set_episode_location,
    AppType

};


fn main() {
    let mut app = App::new();
    configure_default(&mut app);
    set_episode_location(&mut app, AppType::Production);
    app.run();
}
