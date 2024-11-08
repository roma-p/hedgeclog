use bevy::prelude::*;
use hedgeclog::app::builder_bevy_app::{
    configure_default,
    configure_dev,
    set_episode_location,
    AppType

};


fn main() {
    let mut app = App::new();
    configure_default(&mut app);
    configure_dev(&mut app);
    set_episode_location(&mut app, AppType::Testbed);

    app.run();
}
