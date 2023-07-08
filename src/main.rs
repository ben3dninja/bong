use ball::BallPlugin;
use bevy::prelude::*;

mod ball;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BallPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
