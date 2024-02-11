use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    ApplicationSide, DirectionVector, GameState, Heavy, InputReceivedEvent, Lobby, Processing,
    BALL_RADIUS,
};

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

pub(super) struct BallsPlugin;

impl Plugin for BallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HeavyPlugin)
            .add_systems(
                OnEnter(GameState::InGame),
                (
                    dispatch_spawning_locations.run_if(resource_equals(ApplicationSide::Server)),
                    spawn_balls.after(dispatch_spawning_locations),
                )
                    .in_set(Processing),
            )
            .add_systems(
                FixedUpdate,
                (move_balls, jump)
                    .in_set(Processing)
                    .after(choose_direction)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                choose_direction
                    .in_set(Processing)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), despawn_balls);
    }
}

pub(crate) fn dispatch_spawning_locations(mut lobby: ResMut<Lobby>) {
    let locations = vec![Vec3::new(-100., 0., 0.), Vec3::new(100., 0., 0.)];
    for (location, data) in locations.into_iter().zip(lobby.players.values_mut()) {
        data.spawning_location = location;
    }
}

pub(super) fn spawn_balls(mut commands: Commands, mut lobby: ResMut<Lobby>) {
    for data in lobby.players.values_mut() {
        let entity = commands
            .spawn((
                Ball,
                Heavy::default(),
                TransformBundle::from_transform(Transform::from_translation(
                    data.spawning_location,
                )),
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
                Ccd::enabled(),
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

fn choose_direction(
    mut query: Query<&mut DirectionVector, With<Ball>>,
    lobby: Res<Lobby>,
    mut event_reader: EventReader<InputReceivedEvent>,
) {
    for InputReceivedEvent { origin, input } in event_reader.iter() {
        // TODO unwraps
        let mut direction = query
            .get_mut(lobby.players.get(origin).unwrap().entity.unwrap())
            .unwrap();
        *direction = DirectionVector::from(*input);
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
