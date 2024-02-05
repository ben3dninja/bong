use bevy::prelude::*;

use crate::{Displaying, GameState, Processing};

mod ball;
mod scene;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.17, 0.24, 0.31);

pub(super) struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PostUpdate, Displaying.after(Processing))
            .configure_set(OnEnter(GameState::InGame), Displaying.after(Processing))
            .add_systems(Startup, setup_camera)
            .add_systems(
                OnEnter(GameState::InGame),
                ball::display_balls.in_set(Displaying),
            )
            .add_systems(
                Update,
                // Moving display_scene here because it doesn't render the spritebundles properly if called once
                (ball::update_ball_colors, scene::display_scene)
                    .in_set(Displaying)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
}
