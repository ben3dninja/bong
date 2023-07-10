use ball::BallPlugin;
use bevy::prelude::*;
use physics::Physics;
use scene::GameScenePlugin;

mod ball;
mod physics;
mod scene;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((Physics, BallPlugin, GameScenePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
