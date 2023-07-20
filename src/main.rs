use ball::BallPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use scene::GameScenePlugin;

mod ball;
mod scene;

const PPM: f32 = 100.;
const BACKGROUND_COLOR: Color = Color::rgb(0.17, 0.24, 0.31);
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
