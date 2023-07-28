use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{app::AppExit, prelude::*};
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use serde::{Deserialize, Serialize};

use crate::{
    client::channel::ClientChannel, connection_config, DirectionVector, GameState, Heavy,
    NetworkedEntities, PlayerCommand, PlayerInput,
};

use self::{
    ball::{Ball, BallsPlugin},
    channel::ServerChannel,
    scene::GameScenePlugin,
};

mod ball;
pub mod channel;
mod scene;

pub const PPM: f32 = 100.;

pub struct ServerPlugin {
    pub public_addr: SocketAddr,
    pub protocol_id: u64,
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let (server, transport) = self.new_renet_server();
        app.add_state::<GameState>()
            .insert_resource(ServerLobby::default())
            .insert_resource(server)
            .insert_resource(transport)
            .add_plugins(DefaultPlugins)
            .add_plugins((
                RenetServerPlugin,
                NetcodeServerPlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PPM),
            ))
            .add_plugins((BallsPlugin, GameScenePlugin))
            .add_systems(OnEnter(GameState::InGame), start_game)
            .add_systems(OnEnter(GameState::Lobby), start_lobby)
            .add_systems(OnExit(GameState::InGame), stop)
            .add_systems(
                Update,
                (
                    receive_server_events,
                    receive_client_messages_in_game.run_if(in_state(GameState::InGame)),
                    check_player_count,
                ),
            )
            .add_systems(
                PostUpdate,
                send_clients_positions_in_game.run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Default, Resource)]
struct ServerLobby {
    players: HashMap<u64, Option<Entity>>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessage {
    EnterLobby,
    EnterGame { player_ids: Vec<u64> },
    Stop,
    PlayerLeavedInGame { player_id: u64 },
    PlayerHeavinessChange { player_id: u64, heaviness: bool },
}

impl ServerPlugin {
    fn new_renet_server(&self) -> (RenetServer, NetcodeServerTransport) {
        let server = RenetServer::new(connection_config());

        let socket = UdpSocket::bind(self.public_addr).unwrap();
        let server_config = ServerConfig {
            max_clients: 64,
            protocol_id: self.protocol_id,
            public_addr: self.public_addr,
            authentication: ServerAuthentication::Unsecure,
        };
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

        (server, transport)
    }
}

fn receive_server_events(
    mut commands: Commands,
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    state: ResMut<State<GameState>>,
    mut players: ResMut<ServerLobby>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player joined with client id {}", client_id);
                players.players.insert(*client_id, None);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!(
                    "Player with client id {} left with reason \"{}\"",
                    client_id, reason
                );
                match state.get() {
                    GameState::InGame => {
                        server.broadcast_message(
                            ServerChannel::ServerMessages,
                            bincode::serialize(&ServerMessage::PlayerLeavedInGame {
                                player_id: *client_id,
                            })
                            .unwrap(),
                        );
                        // TODO deal with these unwraps
                        commands
                            .get_entity(players.players.get(client_id).unwrap().unwrap())
                            .unwrap()
                            .despawn_recursive();
                    }
                    GameState::Lobby => (),
                }
                players.players.remove(client_id);
            }
        }
    }
}

fn receive_client_messages_in_game(
    mut server: ResMut<RenetServer>,
    lobby: Res<ServerLobby>,
    mut query: Query<(&mut DirectionVector, &mut Heavy), With<Ball>>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            // TODO deal with these unwraps
            let input: PlayerInput = bincode::deserialize(&message).unwrap();
            let (mut direction, _) = query
                .get_mut(lobby.players.get(&client_id).unwrap().unwrap())
                .unwrap();
            *direction = DirectionVector::from(input);
        }
        while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
            // TODO deal with these unwraps
            let player_command: PlayerCommand = bincode::deserialize(&message).unwrap();
            match player_command {
                PlayerCommand::Heavy(heaviness) => {
                    let (_, mut heavy) = query
                        .get_mut(lobby.players.get(&client_id).unwrap().unwrap())
                        .unwrap();
                    heavy.heaviness = heaviness;
                    let message = bincode::serialize(&ServerMessage::PlayerHeavinessChange {
                        player_id: client_id,
                        heaviness,
                    })
                    .unwrap();
                    server.broadcast_message(ServerChannel::ServerMessages, message);
                }
            };
        }
    }
}

fn send_clients_positions_in_game(
    mut server: ResMut<RenetServer>,
    query: Query<&Transform, With<Ball>>,
    lobby: Res<ServerLobby>,
) {
    let mut networked_entities = NetworkedEntities::default();
    for (id, entity) in lobby.players.iter() {
        // TODO unwraps
        let entity = entity.unwrap();
        let translation = query.get(entity).unwrap().translation;
        networked_entities
            .translations
            .insert(*id, (translation.x, translation.y));
    }

    // TODO unwrap
    let message = bincode::serialize(&networked_entities).unwrap();

    server.broadcast_message(ServerChannel::NetworkedEntities, message);
}

fn start_lobby(mut server: ResMut<RenetServer>) {
    // TODO check if this unwrap is safe
    let message = bincode::serialize(&ServerMessage::EnterLobby).unwrap();
    server.broadcast_message(ServerChannel::ServerMessages, message);
    println!("Starting lobby");
}

fn start_game(mut server: ResMut<RenetServer>, lobby: Res<ServerLobby>) {
    let message = bincode::serialize(&ServerMessage::EnterGame {
        player_ids: lobby.players.clone().into_keys().collect(),
    })
    .unwrap();
    server.broadcast_message(ServerChannel::ServerMessages, message);
    println!("Starting game");
}

fn stop(mut exit: EventWriter<AppExit>, mut server: ResMut<RenetServer>) {
    let message = bincode::serialize(&ServerMessage::Stop).unwrap();
    server.broadcast_message(ServerChannel::ServerMessages, message);
    println!("Exiting...");
    exit.send(AppExit);
}

fn check_player_count(
    lobby: Res<ServerLobby>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match state.get() {
        GameState::Lobby => {
            if lobby.players.len() >= 2 {
                next_state.set(GameState::InGame);
            }
        }
        GameState::InGame => {
            if lobby.players.len() < 2 {
                next_state.set(GameState::Lobby);
            }
        }
    }
}
