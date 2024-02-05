use bevy_renet::renet::{ChannelConfig, SendType};

pub enum ClientChannel {
    PlayerInput,
    PlayerHeaviness,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::PlayerInput => 0,
            ClientChannel::PlayerHeaviness => 1,
        }
    }
}
impl ClientChannel {
    pub(crate) fn channels_config() -> Vec<bevy_renet::renet::ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: Self::PlayerInput.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::Unreliable,
            },
            ChannelConfig {
                channel_id: Self::PlayerHeaviness.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::Unreliable,
            },
        ]
    }
}
