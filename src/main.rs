use ball::BallPlugin;
use bevy::prelude::*;
use physics::Physics;

mod ball;
mod physics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Physics)
        .add_plugin(BallPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
