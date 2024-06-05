
use bevy::prelude::*;

pub const EPISODE_LOCATION_GAME: &str = "episodes";
pub const EPISODE_LOCATION_TESTBED: &str = "episodes_testbed";
pub const EPISODE_LOCATION_CONSTRUCT: &str = "episodes_construct";

#[derive(Resource, Default)]
pub struct ResEpisodeLocation {
    pub episode_location: String,
}

// -- PLUGIN -----------------------------------------------------------------
pub struct PluginAppCommon;

impl Plugin for PluginAppCommon {
    fn build(&self, app: &mut App) {
    }
}

