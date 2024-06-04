pub mod asset;
pub mod config;
pub mod editor;
pub mod episode;
pub mod game;
pub mod level;
pub mod app;

use bevy::prelude::*;
use crate::app::builder_bevy_app::add_plugins_default;

fn main() {
    let mut app = App::new();
    add_plugins_default(&mut app);
    app.run();
}
