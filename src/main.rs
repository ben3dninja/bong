use ball::BallPlugin;
use bevy::prelude::*;
use physics::Physics;

mod ball;
mod physics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((Physics, BallPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
