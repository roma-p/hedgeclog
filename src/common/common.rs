use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct GridPosition {
    pub x : usize,
    pub z : usize,
}

