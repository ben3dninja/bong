use bevy::prelude::*;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use bong::{ball::BallPlugin, scene::GameScenePlugin, BACKGROUND_COLOR, PPM};

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PPM),
            // RapierDebugRenderPlugin::default(),
            BallPlugin,
            GameScenePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
