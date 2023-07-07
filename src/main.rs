use ball::BallPlugin;
use bevy::prelude::*;

mod ball;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BallPlugin)
        .run();
}
