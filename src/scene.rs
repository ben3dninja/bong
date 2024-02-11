use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{GameState, Processing, WALL_HALF_HEIGHT, WALL_HALF_WIDTH, WALL_TRANSFORM};

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_scene.in_set(Processing));
    }
}

#[derive(Component)]
pub struct Wall;

pub fn spawn_scene(mut commands: Commands) {
    commands.spawn((
        Wall,
        TransformBundle::from_transform(WALL_TRANSFORM),
        RigidBody::Fixed,
        Collider::cuboid(WALL_HALF_WIDTH, WALL_HALF_HEIGHT),
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        },
        Restitution {
            coefficient: 0.99,
            combine_rule: CoefficientCombineRule::Max,
        },
    ));
}
