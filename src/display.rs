use bevy::prelude::*;

mod balls;
mod walls;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.17, 0.24, 0.31);

pub struct GameDisplayPlugin;

impl Plugin for GameDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, balls::create_ball_mesh).chain())
            .add_systems(PostStartup, (walls::display_walls, balls::display_balls))
            .add_systems(PostUpdate, balls::change_color);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
}
