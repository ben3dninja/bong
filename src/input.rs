use bevy::prelude::*;
use serde::{Deserialize, Serialize};

mod direction;

pub use direction::*;

#[derive(Default, Serialize, Deserialize)]
pub struct PlayerInput {
    ball_direction: Direction,
    heavier: bool,
}

#[derive(Component)]
pub struct PlayerControlled;
