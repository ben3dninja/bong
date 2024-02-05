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

pub const PPM: f32 = 100.;
pub const FIXED_DT: f32 = 0.02;

pub const PHYSICS_DT: f32 = 1. * FIXED_DT;
pub const SUBSTEPS: usize = 1;

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

#[derive(Debug, PartialEq, Eq, Resource)]
pub enum ApplicationSide {
    Server,
    Client,
}

/// The `Lobby` stores the set of players identified by a unique `u64`
/// and their respective entities and initial data
#[derive(Clone, Debug, Default, Serialize, Deserialize, Resource)]
pub struct Lobby {
    players: HashMap<u64, PlayerData>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerData {
    spawning_location: Vec3,
    entity: Option<Entity>,
}

#[derive(Copy, Clone, Component, Debug, Default, Serialize, Deserialize)]
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

impl From<Vec2> for DirectionVector {
    fn from(value: Vec2) -> Self {
        DirectionVector(value.normalize_or_zero())
    }
}

impl From<DirectionVector> for Vec2 {
    fn from(value: DirectionVector) -> Self {
        value.0
    }
}

#[derive(Component, Debug, Default)]
pub struct Heavy {
    pub heaviness: bool,
    pub heavy_timer: Stopwatch,
}

#[derive(Event)]
pub struct InputReceivedEvent {
    origin: u64,
    input: PlayerInput,
}

#[derive(Event)]
pub struct HeavinessReceivedEvent {
    origin: u64,
    heaviness: bool,
}

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        server_channels_config: ServerChannel::channels_config(),
        client_channels_config: ClientChannel::channels_config(),
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
struct Receiving;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
struct Processing;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
struct Sending;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
struct Displaying;
