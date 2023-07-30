use bevy::prelude::*;

const WALL_COLOR: Color = Color::rgb(0.31, 0.49, 0.67);
use crate::{GameState, WALL_HALF_HEIGHT, WALL_HALF_WIDTH, WALL_TRANSFORM};

#[derive(Component)]
struct Wall;

pub(super) struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), display_scene);
    }
}

fn display_scene(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: WALL_COLOR,
            custom_size: Some(Vec2::new(WALL_HALF_WIDTH * 2., WALL_HALF_HEIGHT * 2.)),
            ..default()
        },
        transform: WALL_TRANSFORM,
        ..default()
    });
}
