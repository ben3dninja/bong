use bevy::prelude::*;
use bong::server::ServerPlugin;

fn main() {
    App::new()
        .add_plugins(ServerPlugin {
            public_addr: "127.0.0.1:5000".parse().unwrap(),
            protocol_id: 1,
        })
        .run();
}
