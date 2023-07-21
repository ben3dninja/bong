use bevy::prelude::*;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use bong::{
    ball::BallPlugin, display::GameDisplayPlugin, input::PlayerInputPlugin, scene::GameScenePlugin,
    PPM,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PPM),
            // RapierDebugRenderPlugin::default(),
            PlayerInputPlugin,
            BallPlugin,
            GameScenePlugin,
            GameDisplayPlugin,
        ))
        .run();
}
