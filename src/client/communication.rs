use std::collections::HashMap;

use bevy::{app::AppExit, prelude::*};
use bevy_renet::renet::RenetClient;

use crate::{
    ball::Ball,
    client::channel::ClientChannel,
    server::{channel::ServerChannel, ServerMessage},
    DirectionVector, GameState, HeavinessReceivedEvent, Heavy, InputReceivedEvent, Lobby,
    PlayerInput, Receiving, Sending,
};

const KEY_UP: KeyCode = KeyCode::W;
const KEY_DOWN: KeyCode = KeyCode::S;
const KEY_LEFT: KeyCode = KeyCode::A;
const KEY_RIGHT: KeyCode = KeyCode::D;
const KEY_HEAVY: KeyCode = KeyCode::Space;

// const MOVEMENT_KEYS: [KeyCode; 4] = [KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT];
pub struct ClientCommunicationPlugin;

impl Plugin for ClientCommunicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputReceivedEvent>()
            .add_event::<HeavinessReceivedEvent>()
            .add_systems(
                FixedUpdate,
                (
                    (send_player_input, send_player_heaviness).in_set(Sending),
                    // receive_player_inputs,
                    // receive_player_heaviness,
                    (receive_networked_entities).in_set(Receiving),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(FixedUpdate, receive_server_message.in_set(Receiving));
    }
}

pub fn receive_player_inputs(
    mut client: ResMut<RenetClient>,
    mut event_writer: EventWriter<InputReceivedEvent>,
) {
    while let Some(message) = client.receive_message(ServerChannel::PlayerInput) {
        // TODO unwraps
        let (origin, input): (u64, PlayerInput) = bincode::deserialize(&message).unwrap();
        event_writer.send(InputReceivedEvent { origin, input });
    }
}

pub fn receive_player_heaviness(
    mut client: ResMut<RenetClient>,
    mut event_writer: EventWriter<HeavinessReceivedEvent>,
) {
    while let Some(message) = client.receive_message(ServerChannel::PlayerHeaviness) {
        // TODO unwraps
        let (origin, heaviness): (u64, bool) = bincode::deserialize(&message).unwrap();
        event_writer.send(HeavinessReceivedEvent { origin, heaviness });
    }
}

fn receive_server_message(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut next_state: ResMut<NextState<GameState>>,
    mut lobby: ResMut<Lobby>,
    mut exit: EventWriter<AppExit>,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        // TODO unwrap
        let message: ServerMessage = bincode::deserialize(&message).unwrap();
        match message {
            ServerMessage::EnterLobby => next_state.set(GameState::Lobby),
            ServerMessage::EnterGame { players } => {
                lobby.players = players;
                // entities are server entities but since they are immediately written away when balls are spawned this is not a problem
                next_state.set(GameState::InGame);
            }
            ServerMessage::Stop => exit.send(AppExit),
            ServerMessage::PlayerLeavedInGame { player_id } => {
                // TODO unwraps
                let entity = lobby.players.get(&player_id).unwrap().entity.unwrap();
                commands.get_entity(entity).unwrap().despawn_recursive()
            }
        }
    }
}

fn send_player_input(mut client: ResMut<RenetClient>, k_in: Res<Input<KeyCode>>) {
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
    let message = bincode::serialize(&PlayerInput { direction }).unwrap();
    client.send_message(ClientChannel::PlayerInput, message);
}

fn send_player_heaviness(mut client: ResMut<RenetClient>, k_in: Res<Input<KeyCode>>) {
    let message = bincode::serialize(&k_in.pressed(KEY_HEAVY)).unwrap();
    client.send_message(ClientChannel::PlayerHeaviness, message)
}

pub(crate) fn receive_networked_entities(
    mut client: ResMut<RenetClient>,
    lobby: Res<Lobby>,
    mut query: Query<(&mut Transform, &mut DirectionVector, &mut Heavy), With<Ball>>,
) {
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let map: HashMap<u64, (Vec3, DirectionVector, bool)> =
            bincode::deserialize(&message).unwrap();
        for (id, (translation, new_direction, heaviness)) in map {
            let entity = lobby.players.get(&id).unwrap().entity.unwrap();
            let (mut transform, mut direction, mut heavy) = query.get_mut(entity).unwrap();
            transform.translation = translation;
            *direction = new_direction;
            heavy.heaviness = heaviness;
        }
    }
}
