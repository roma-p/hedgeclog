use bevy::prelude::*;
use crate::config::StateGlobal;

pub struct PluginGame;

impl Plugin for PluginGame{
    fn build(&self, app: &mut App){
        app.add_systems(
            Update, 
            (
                user_input_game_global.run_if(in_state(StateGlobal::Game)),
            )
        );
    }
}


fn user_input_game_global(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state_global: ResMut<NextState<StateGlobal>>,

) {
    if keyboard_input.pressed(KeyCode::KeyE) {
        state_global.set(StateGlobal::Editor); 
    }
    // add move hedghog, restart, undo.
}
