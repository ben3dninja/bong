use bevy::prelude::*;

const WALL_COLOR: Color = Color::rgb(0.31, 0.49, 0.67);
use crate::{scene::Wall, WALL_HALF_HEIGHT, WALL_HALF_WIDTH, WALL_TRANSFORM};

/// Adds display components to each entity in the scene (excluding the balls)
pub(super) fn display_scene(
    mut commands: Commands,
    query: Query<Entity, (With<Wall>, Without<Sprite>)>,
) {
    for entity in query.iter() {
        commands.get_entity(entity).unwrap().insert(SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(WALL_HALF_WIDTH * 2., WALL_HALF_HEIGHT * 2.)),
                ..default()
            },
            transform: WALL_TRANSFORM,
            ..default()
        });
    }
}
