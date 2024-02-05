use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{app::AppExit, diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode};
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use renet_visualizer::RenetServerVisualizer;
use serde::{Deserialize, Serialize};

use crate::{
    ball::BallsPlugin, connection_config, display::DisplayPlugin, scene::GameScenePlugin,
    ApplicationSide, GameState, Lobby, PlayerData, Processing, Receiving, Sending, FIXED_DT,
    PHYSICS_DT, PPM, SUBSTEPS,
};

use self::{channel::ServerChannel, communication::ServerCommunicationPlugin};

pub mod channel;
pub mod communication;

pub struct ServerPlugin {
    pub public_addr: SocketAddr,
    pub protocol_id: u64,
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let (server, transport) = self.new_renet_server();
        app.add_state::<GameState>()
            .insert_resource(Lobby::default())
            .insert_resource(ApplicationSide::Server)
            .insert_resource(server)
            .insert_resource(transport)
            .insert_resource(RapierConfiguration {
                timestep_mode: TimestepMode::Fixed {
                    dt: PHYSICS_DT,
                    substeps: SUBSTEPS,
                },
                ..default()
            })
            .add_plugins(DefaultPlugins)
            .insert_resource(FixedTime::new_from_secs(FIXED_DT))
            .add_plugins((
                RenetServerPlugin,
                NetcodeServerPlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PPM),
            ))
            .add_plugins((
                // FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::default(),
                EguiPlugin,
            ))
            .insert_resource(RenetServerVisualizer::<200>::default())
            .add_plugins((BallsPlugin, GameScenePlugin))
            .add_plugins(DisplayPlugin)
            .add_plugins(ServerCommunicationPlugin)
            .configure_sets(FixedUpdate, (Receiving, Processing, Sending).chain())
            .configure_sets(
                OnEnter(GameState::InGame),
                (Receiving, Processing, Sending).chain(),
            )
            .add_systems(OnEnter(GameState::InGame), start_game.in_set(Sending))
            .add_systems(OnEnter(GameState::Lobby), start_lobby.in_set(Sending))
            .add_systems(OnExit(GameState::InGame), stop.in_set(Sending))
            .add_systems(Update, update_visualizer_system)
            .add_systems(
                FixedUpdate,
                (receive_server_events.in_set(Receiving), check_player_count),
            );
    }
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessage {
    EnterLobby,
    EnterGame { players: HashMap<u64, PlayerData> },
    Stop,
    PlayerLeavedInGame { player_id: u64 },
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
    mut players: ResMut<Lobby>,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player joined with client id {}", client_id);
                players.players.insert(*client_id, PlayerData::default());
                visualizer.add_client(*client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!(
                    "Player with client id {} left with reason \"{}\"",
                    client_id, reason
                );
                visualizer.remove_client(*client_id);
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
                            .get_entity(players.players.get(client_id).unwrap().entity.unwrap())
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

fn start_lobby(mut server: ResMut<RenetServer>) {
    // TODO check if this unwrap is safe
    let message = bincode::serialize(&ServerMessage::EnterLobby).unwrap();
    server.broadcast_message(ServerChannel::ServerMessages, message);
    println!("Starting lobby");
}

fn start_game(mut server: ResMut<RenetServer>, lobby: Res<Lobby>) {
    let message = bincode::serialize(&ServerMessage::EnterGame {
        players: lobby.players.clone(),
    })
    .unwrap();
    server.broadcast_message(ServerChannel::ServerMessages, message);
    println!("Starting game...");
}

fn stop(mut exit: EventWriter<AppExit>, mut server: ResMut<RenetServer>) {
    let message = bincode::serialize(&ServerMessage::Stop).unwrap();
    server.broadcast_message(ServerChannel::ServerMessages, message);
    println!("Exiting...");
    exit.send(AppExit);
}

fn check_player_count(
    lobby: Res<Lobby>,
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

fn update_visualizer_system(
    mut egui_contexts: EguiContexts,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
    server: Res<RenetServer>,
) {
    visualizer.update(&server);
    visualizer.show_window(egui_contexts.ctx_mut());
}
