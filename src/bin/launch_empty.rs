
use bevy::prelude::*;
// use crate::app::builder_bevy_app::{add_plugins_default, add_plugins_dev};

fn main() {
    let mut app = App::new();
    add_plugins_default(&mut app);
    add_plugins_dev(&mut app);
    app.run();
}
