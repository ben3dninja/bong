use bevy::prelude::*;
use bong::ClientPlugin;

fn main() {
    App::new()
        .add_plugins(ClientPlugin {
            server_addr: "127.0.0.1:5000".parse().unwrap(),
            protocol_id: 1,
            socket_addr: "127.0.0.1:0".parse().unwrap(),
        })
        .run();
}
