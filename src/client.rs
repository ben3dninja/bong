use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode};
use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use renet_visualizer::RenetClientVisualizer;

use crate::{
    ball::BallsPlugin, connection_config, display::DisplayPlugin, scene::GameScenePlugin,
    ApplicationSide, GameState, Lobby, Processing, Receiving, Sending, FIXED_DT, PHYSICS_DT, PPM,
    SUBSTEPS,
};

use self::communication::ClientCommunicationPlugin;

pub mod channel;
pub mod communication;

pub struct ClientPlugin {
    pub server_addr: SocketAddr,
    pub socket_addr: SocketAddr,
    pub protocol_id: u64,
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let (client, transport) = self.new_renet_client();
        app.add_state::<GameState>()
            .insert_resource(Lobby::default())
            .insert_resource(client)
            .insert_resource(ApplicationSide::Client)
            .insert_resource(transport)
            .insert_resource(FixedTime::new_from_secs(FIXED_DT))
            .insert_resource(RapierConfiguration {
                timestep_mode: TimestepMode::Fixed {
                    dt: PHYSICS_DT,
                    substeps: SUBSTEPS,
                },
                ..default()
            })
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Client".to_owned(),
                    ..default()
                }),
                ..default()
            }))
            .add_plugins((
                RenetClientPlugin,
                NetcodeClientPlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PPM),
            ))
            .add_plugins((
                // FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::default(),
                EguiPlugin,
            ))
            .configure_sets(FixedUpdate, (Sending, Receiving, Processing).chain())
            .configure_sets(
                OnEnter(GameState::InGame),
                (Sending, Receiving, Processing).chain(),
            )
            .add_plugins(ClientCommunicationPlugin)
            .add_plugins((BallsPlugin, GameScenePlugin, DisplayPlugin))
            .add_systems(Update, update_visualizer_system)
            .insert_resource(RenetClientVisualizer::<200>::default());
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

fn update_visualizer_system(
    mut egui_contexts: EguiContexts,
    mut visualizer: ResMut<RenetClientVisualizer<200>>,
    client: Res<RenetClient>,
    mut show_visualizer: Local<bool>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    visualizer.add_network_info(client.network_info());
    if keyboard_input.just_pressed(KeyCode::F1) {
        *show_visualizer = !*show_visualizer;
    }
    if *show_visualizer {
        visualizer.show_window(egui_contexts.ctx_mut());
    }
}
