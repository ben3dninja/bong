use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    input::{DirectionVector, PlayerControlled},
    scene::Wall,
};

use self::heavier::HeavyPlugin;

use super::Ball;

pub mod heavier;

/// Force applied to the ball when a key is pressed, in  kilogram pixel per second squared.
const MOVEMENT_FORCE: f32 = 30.;
const JUMP_SPEED: f32 = 25.;
/// Y component of the direction vector that triggers jumping
const JUMP_THRESHOLD: f32 = 0.2;

const KEY_UP: KeyCode = KeyCode::W;
const KEY_DOWN: KeyCode = KeyCode::S;
const KEY_LEFT: KeyCode = KeyCode::A;
const KEY_RIGHT: KeyCode = KeyCode::D;

pub struct BallMovement;

impl Plugin for BallMovement {
    fn build(&self, app: &mut App) {
        app.add_plugins(HeavyPlugin)
            .add_systems(Update, (set_direction, move_balls, jump).chain());
    }
}

fn set_direction(
    k_in: Res<Input<KeyCode>>,
    mut query: Query<&mut DirectionVector, (With<Ball>, With<PlayerControlled>)>,
) {
    let mut direction = Vec2::ZERO;
    if k_in.pressed(KEY_UP) {
        direction.y += 1.;
    }
    if k_in.pressed(KEY_LEFT) {
        direction.x -= 1.;
    }
    if k_in.pressed(KEY_DOWN) {
        direction.y -= 1.;
    }
    if k_in.pressed(KEY_RIGHT) {
        direction.x += 1.;
    }
    *query.single_mut() = DirectionVector::new_normalize(direction);
}

fn move_balls(mut query: Query<(&mut ExternalForce, &DirectionVector), With<Ball>>) {
    for (mut force, direction) in query.iter_mut() {
        force.force = MOVEMENT_FORCE * **direction;
    }
}

fn jump(
    mut ball_query: Query<(Entity, &DirectionVector, &mut ExternalImpulse), With<Ball>>,
    wall_query: Query<Entity, With<Wall>>,
    ctx: Res<RapierContext>,
) {
    let (ball, direction, mut ball_imp) = ball_query.single_mut();
    let wall = wall_query.single();
    if let Some(contact_pair) = ctx.contact_pair(ball, wall) {
        if direction.y > JUMP_THRESHOLD && contact_pair.has_any_active_contacts() {
            ball_imp.impulse = Vec2::Y * JUMP_SPEED;
        }
    }
}
