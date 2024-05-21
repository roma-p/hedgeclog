use bevy::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct GridPosition {
    pub x : usize,
    pub z : usize,
}

