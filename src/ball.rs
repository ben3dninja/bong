use bevy::{prelude::*, time::Stopwatch};
use bevy_rapier2d::prelude::*;

use crate::input::{DirectionVector, PlayerControlled};

use self::movement::BallMovement;

pub mod movement;

/// Ball radius, in pixels
pub const BALL_RADIUS: f32 = 20.;
/// Ball starting position, in pixels
pub const BALL_STARTING_TRANSFORM: Transform = Transform::from_xyz(0., 0., 0.);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_ball)
            .add_plugins(BallMovement);
    }
}

#[derive(Default, Component)]
pub struct Ball {
    pub is_heavy: bool,
    pub heavy_timer: Stopwatch,
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        Ball::default(),
        PlayerControlled,
        TransformBundle::from_transform(BALL_STARTING_TRANSFORM),
        DirectionVector::default(),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::ball(BALL_RADIUS),
        ExternalForce::default(),
        ExternalImpulse::default(),
        Ccd::enabled(),
        GravityScale(4.5),
        AdditionalMassProperties::default(),
        Sleeping::disabled(),
    ));
}
