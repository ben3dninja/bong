use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{app::AppExit, prelude::*};
use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};

use crate::{
    connection_config,
    server::{channel::ServerChannel, ServerMessage},
    GameState, Heavy, NetworkedEntities, PlayerCommand, PlayerInput,
};

use self::{
    ball::{Ball, BallsPlugin},
    channel::ClientChannel,
    scene::GameScenePlugin,
};

pub mod channel;

mod ball;
mod scene;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.17, 0.24, 0.31);

const KEY_UP: KeyCode = KeyCode::W;
const KEY_DOWN: KeyCode = KeyCode::S;
const KEY_LEFT: KeyCode = KeyCode::A;
const KEY_RIGHT: KeyCode = KeyCode::D;
const KEY_HEAVY: KeyCode = KeyCode::Space;

const MOVEMENT_KEYS: [KeyCode; 4] = [KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT];

#[derive(Default, Resource)]
struct ClientLobby {
    players: HashMap<u64, Option<Entity>>,
}

pub struct ClientPlugin {
    pub server_addr: SocketAddr,
    pub socket_addr: SocketAddr,
    pub protocol_id: u64,
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let (client, transport) = self.new_renet_client();
        app.add_state::<GameState>()
            .insert_resource(ClientLobby::default())
            .insert_resource(client)
            .insert_resource(transport)
            .add_plugins(DefaultPlugins)
            .add_plugins((RenetClientPlugin, NetcodeClientPlugin))
            .add_plugins((BallsPlugin, GameScenePlugin))
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (
                    receive_server_message,
                    receive_entities_in_game.run_if(in_state(GameState::InGame)),
                    send_server_message,
                ),
            );
    }
}

impl ClientPlugin {
    fn new_renet_client(&self) -> (RenetClient, NetcodeClientTransport) {
        let client = RenetClient::new(connection_config());

        let socket = UdpSocket::bind(self.socket_addr).unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: self.protocol_id,
            server_addr: self.server_addr,
            user_data: None,
        };

        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

        (client, transport)
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
}

fn receive_server_message(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut next_state: ResMut<NextState<GameState>>,
    mut lobby: ResMut<ClientLobby>,
    mut exit: EventWriter<AppExit>,
    mut query: Query<&mut Heavy, With<Ball>>,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        // TODO unwrap
        let message: ServerMessage = bincode::deserialize(&message).unwrap();
        match message {
            ServerMessage::EnterLobby => next_state.set(GameState::Lobby),
            ServerMessage::EnterGame { player_ids } => {
                for id in player_ids {
                    lobby.players.insert(id, None);
                }
                next_state.set(GameState::InGame);
            }
            ServerMessage::Stop => exit.send(AppExit),
            ServerMessage::PlayerLeavedInGame { player_id } => {
                // TODO unwraps
                let entity = lobby.players.get(&player_id).unwrap().unwrap();
                commands.get_entity(entity).unwrap().despawn_recursive()
            }
            ServerMessage::PlayerHeavinessChange {
                player_id,
                heaviness,
            } => {
                let entity = lobby.players.get(&player_id).unwrap().unwrap();
                query.get_mut(entity).unwrap().heaviness = heaviness;
            }
        }
    }
}

fn receive_entities_in_game(
    lobby: Res<ClientLobby>,
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
) {
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        // TODO unwraps
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();
        for (id, (x, y)) in networked_entities.translations {
            let entity = lobby.players.get(&id).unwrap().unwrap();
            commands
                .get_entity(entity)
                .unwrap()
                .insert(Transform::from_xyz(x, y, 0.));
        }
    }
}

fn send_server_message(mut client: ResMut<RenetClient>, k_in: Res<Input<KeyCode>>) {
    if k_in.any_just_pressed(MOVEMENT_KEYS) || k_in.any_just_released(MOVEMENT_KEYS) {
        let mut direction = Vec2::ZERO;
        for key in k_in.get_pressed() {
            direction += match *key {
                KEY_UP => Vec2::Y,
                KEY_DOWN => Vec2::NEG_Y,
                KEY_RIGHT => Vec2::X,
                KEY_LEFT => Vec2::NEG_X,
                _ => Vec2::ZERO,
            }
        }
        direction = direction.normalize_or_zero();
        let message = bincode::serialize(&PlayerInput { direction }).unwrap();
        client.send_message(ClientChannel::Input, message);
    }
    if k_in.just_pressed(KEY_HEAVY) || k_in.just_released(KEY_HEAVY) {
        let message = bincode::serialize(&PlayerCommand::Heavy(k_in.pressed(KEY_HEAVY))).unwrap();
        client.send_message(ClientChannel::Command, message)
    }
}
