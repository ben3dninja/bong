use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const WALL_TRANSFORM: Transform = Transform::from_xyz(0., -200., 0.);
pub const WALL_HALF_WIDTH: f32 = 300.;
pub const WALL_HALF_HEIGHT: f32 = 5.;

#[derive(Component)]
pub struct Wall;

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_scene);
    }
}

fn create_scene(mut commands: Commands) {
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
