use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const WALL_COLOR: Color = Color::rgb(0.31, 0.49, 0.67);

const WALL_POSITION: Vec3 = Vec3::new(0., -200., 0.);
const WALL_HALF_WIDTH: f32 = 300.;
const WALL_HALF_HEIGHT: f32 = 5.;

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
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(WALL_HALF_WIDTH * 2., WALL_HALF_HEIGHT * 2.)),
                ..default()
            },
            transform: Transform {
                translation: WALL_POSITION,
                ..default()
            },
            ..default()
        },
        Wall,
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
