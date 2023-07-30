use std::{collections::HashMap, time::Duration};

use bevy::{prelude::*, time::Stopwatch};

pub mod client;
pub mod server;

mod ball;
mod display;
mod scene;

use bevy_renet::renet::ConnectionConfig;
use client::channel::ClientChannel;
pub use client::ClientPlugin;
use derive_more::Mul;
use serde::{Deserialize, Serialize};
use server::channel::ServerChannel;
pub use server::ServerPlugin;

pub const BALL_RADIUS: f32 = 20.;
pub const HEAVINESS_DURATION: Duration = Duration::new(5, 0);

pub const WALL_TRANSFORM: Transform = Transform::from_xyz(0., -200., 0.);
pub const WALL_HALF_WIDTH: f32 = 300.;
pub const WALL_HALF_HEIGHT: f32 = 5.;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    Lobby,
    InGame,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Resource)]
pub struct Lobby {
    players: HashMap<u64, PlayerData>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerData {
    spawning_location: Vec3,
    entity: Option<Entity>,
}

#[derive(Debug, Resource, Eq, PartialEq)]
pub enum ApplicationSide {
    Server,
    Client,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerInput {
    direction: Vec2,
}

#[derive(Copy, Clone, Component, Debug, Default, Serialize, Deserialize, Mul)]
pub struct DirectionVector(Vec2);

impl From<PlayerInput> for DirectionVector {
    fn from(value: PlayerInput) -> Self {
        DirectionVector(value.direction.normalize_or_zero())
    }
}

impl From<DirectionVector> for Vec2 {
    fn from(value: DirectionVector) -> Self {
        value.0
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PlayerCommand {
    Heavy(bool),
}

#[derive(Component, Debug, Default)]
pub struct Heavy {
    pub heaviness: bool,
    pub heavy_timer: Stopwatch,
}

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        server_channels_config: ServerChannel::channels_config(),
        client_channels_config: ClientChannel::channels_config(),
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NetworkedEntities {
    pub translations: HashMap<u64, (f32, f32)>,
}
