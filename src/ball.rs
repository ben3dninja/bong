use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{ApplicationSide, DirectionVector, GameState, Heavy, Lobby, BALL_RADIUS};

use self::heavy::HeavyPlugin;

use crate::scene::Wall;

/// Force applied to the ball when a key is pressed, in  kilogram pixel per second squared.
const MOVEMENT_FORCE: f32 = 30.;
const JUMP_SPEED: f32 = 25.;
/// Y component of the direction vector that triggers jumping
const JUMP_THRESHOLD: f32 = 0.2;

mod heavy;

#[derive(Component)]
pub(super) struct Ball;

#[derive(Resource)]
pub(super) struct SpawningLocations {
    locations: Vec<Vec3>,
}

pub(super) struct BallsPlugin;

impl Plugin for BallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HeavyPlugin)
            .add_systems(OnEnter(GameState::InGame), spawn_balls)
            .add_systems(
                Update,
                (move_balls, jump)
                    .after(receive_player_inputs)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), despawn_balls);
    }
}

pub(super) fn spawn_balls(
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    spawning_locations: Res<SpawningLocations>,
    side: Res<ApplicationSide>,
) {
    let mut locations = spawning_locations.locations.into_iter();
    for data in lobby.players.values_mut() {
        let entity = commands
            .spawn((
                Ball,
                Heavy::default(),
                TransformBundle::from_transform(Transform::from_translation(
                    locations.next().unwrap(),
                )), // TODO fix unwraping
                DirectionVector::default(),
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Collider::ball(BALL_RADIUS),
                ExternalForce::default(),
                ExternalImpulse::default(),
                GravityScale(4.5),
                AdditionalMassProperties::default(),
                Sleeping::disabled(),
                Restitution {
                    coefficient: 1.,
                    combine_rule: CoefficientCombineRule::Min,
                },
            ))
            .id();
        data.entity = Some(entity);
    }
}

pub(super) fn despawn_balls(mut commands: Commands, balls: Query<Entity, With<Ball>>) {
    for ball in balls.iter() {
        commands.get_entity(ball).unwrap().despawn_recursive();
    }
}

fn move_balls(mut query: Query<(&mut ExternalForce, &DirectionVector), With<Ball>>) {
    for (mut force, direction) in query.iter_mut() {
        force.force = (*direction * MOVEMENT_FORCE).into();
    }
}

fn jump(
    mut ball_query: Query<(Entity, &DirectionVector, &mut ExternalImpulse), With<Ball>>,
    wall_query: Query<Entity, With<Wall>>,
    ctx: Res<RapierContext>,
) {
    for (ball, direction, mut ball_imp) in ball_query.iter_mut() {
        let wall = wall_query.single();
        if let Some(contact_pair) = ctx.contact_pair(ball, wall) {
            if Vec2::from(*direction).y > JUMP_THRESHOLD && contact_pair.has_any_active_contacts() {
                ball_imp.impulse = Vec2::Y * JUMP_SPEED;
            }
        }
    }
}

fn create_spawning_locations() -> Vec<Vec3> {
    vec![Vec3::new(-100., 0., 0.), Vec3::new(100., 0., 0.)]
}
