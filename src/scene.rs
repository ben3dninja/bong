use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{GameState, WALL_HALF_HEIGHT, WALL_HALF_WIDTH, WALL_TRANSFORM};

pub(super) struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_scene);
    }
}

#[derive(Component)]
pub struct Wall;

fn spawn_scene(mut commands: Commands) {
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
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Max,
        },
    ));
}
