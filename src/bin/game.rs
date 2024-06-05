use bevy::prelude::*;
use hedgeclog::app::builder_bevy_app::add_plugins_default;


fn main() {
    let mut app = App::new();
    add_plugins_default(&mut app);
    app.run();
}
