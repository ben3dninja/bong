use std::collections::HashMap;

use bevy::prelude::*;
use bevy_renet::renet::RenetServer;

use crate::{
    ball::Ball, client::channel::ClientChannel, server::channel::ServerChannel, DirectionVector,
    GameState, HeavinessReceivedEvent, Heavy, InputReceivedEvent, Lobby, PlayerInput,
};

use super::{Receiving, Sending};

pub struct ServerCommunicationPlugin;

impl Plugin for ServerCommunicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputReceivedEvent>()
            .add_event::<HeavinessReceivedEvent>()
            .add_systems(
                FixedUpdate,
                (
                    // broadcast_players_inputs,
                    // broadcast_players_heaviness,
                    (receive_player_inputs, receive_player_heaviness).in_set(Receiving),
                    broadcast_networked_entities.in_set(Sending), // run_if(on_fixed_timer(Duration::from_millis(100))),
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

pub fn receive_player_inputs(
    mut server: ResMut<RenetServer>,
    lobby: Res<Lobby>,
    mut event_writer: EventWriter<InputReceivedEvent>,
) {
    for &origin in lobby.players.keys() {
        while let Some(message) = server.receive_message(origin, ClientChannel::PlayerInput) {
            // TODO unwrap
            let input: PlayerInput = bincode::deserialize(&message).unwrap();
            event_writer.send(InputReceivedEvent { origin, input });
        }
    }
}

pub fn receive_player_heaviness(
    mut server: ResMut<RenetServer>,
    lobby: Res<Lobby>,
    mut event_writer: EventWriter<HeavinessReceivedEvent>,
) {
    for &origin in lobby.players.keys() {
        while let Some(message) = server.receive_message(origin, ClientChannel::PlayerHeaviness) {
            // TODO unwraps
            let heaviness: bool = bincode::deserialize(&message).unwrap();
            event_writer.send(HeavinessReceivedEvent { origin, heaviness });
        }
    }
}

pub fn broadcast_players_inputs(
    mut server: ResMut<RenetServer>,
    mut event_reader: EventReader<InputReceivedEvent>,
) {
    for InputReceivedEvent { origin, input } in event_reader.iter() {
        let message = bincode::serialize(&(origin, input)).unwrap();
        server.broadcast_message(ServerChannel::PlayerInput, message);
    }
}

pub fn broadcast_players_heaviness(
    mut server: ResMut<RenetServer>,
    mut event_reader: EventReader<HeavinessReceivedEvent>,
) {
    for HeavinessReceivedEvent { origin, heaviness } in event_reader.iter() {
        let message = bincode::serialize(&(origin, heaviness)).unwrap();
        server.broadcast_message(ServerChannel::PlayerHeaviness, message);
    }
}

pub(crate) fn broadcast_networked_entities(
    mut server: ResMut<RenetServer>,
    lobby: Res<Lobby>,
    query: Query<(&Transform, &DirectionVector, &Heavy), With<Ball>>,
) {
    let mut map = HashMap::new();
    for (id, data) in lobby.players.iter() {
        let (transform, direction, heavy) = query.get(data.entity.unwrap()).unwrap();
        map.insert(id, (transform.translation, direction, heavy.heaviness));
    }
    let message = bincode::serialize(&map).unwrap();
    server.broadcast_message(ServerChannel::NetworkedEntities, message);
}
