use std::time::Duration;

use bevy_renet::renet::{ChannelConfig, SendType};

pub enum ServerChannel {
    ServerMessages,
    PlayerInput,
    PlayerHeaviness,
    NetworkedEntities,
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::ServerMessages => 0,
            ServerChannel::PlayerInput => 1,
            ServerChannel::PlayerHeaviness => 2,
            ServerChannel::NetworkedEntities => 3,
        }
    }
}

impl ServerChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: Self::ServerMessages.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(200),
                },
            },
            ChannelConfig {
                channel_id: Self::PlayerInput.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::Unreliable,
            },
            ChannelConfig {
                channel_id: Self::PlayerHeaviness.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::Unreliable,
            },
            ChannelConfig {
                channel_id: Self::NetworkedEntities.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::Unreliable,
            },
        ]
    }
}
