use bevy::prelude::*;

use crate::physics::{Mass, Velocity};

use super::Ball;

/// Force applied to the ball when a key is pressed, in  ballmass pixel per second squared.
const MOVEMENT_FORCE: f32 = 500.;

pub struct BallMovement;

impl Plugin for BallMovement {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_keys);
    }
}

fn movement_keys(
    mut query: Query<(&mut Velocity, &Mass), With<Ball>>,
    k_in: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::default();
    if k_in.pressed(KeyCode::W) {
        direction.y += 1.;
    }
    if k_in.pressed(KeyCode::A) {
        direction.x -= 1.;
    }
    if k_in.pressed(KeyCode::S) {
        direction.y -= 1.;
    }
    if k_in.pressed(KeyCode::D) {
        direction.x += 1.;
    }
    let (mut velocity, mass) = query.single_mut();
    // F = m dv/dt -> dv = F/m dt
    **velocity += direction * MOVEMENT_FORCE / **mass * time.delta_seconds();
}
