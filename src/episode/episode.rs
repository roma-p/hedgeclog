
use bevy::prelude::*;
use crate::episode::episode_definition::PluginEpsiodeDefinition;
// use crate::episode::load_run_episode

// -- PLUGIN -----------------------------------------------------------------

pub struct PluginEpsiode;

impl Plugin for PluginEpsiode {
    fn build(&self, app: &mut App) {
        app.add_plugins(PluginEpsiodeDefinition);
    }
}

