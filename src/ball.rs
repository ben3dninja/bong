use bevy::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_ball);
    }
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn(Ball);
}

#[derive(Component)]
struct Ball;
