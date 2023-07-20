use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::scene::Wall;

use super::Ball;


/// Force applied to the ball when a key is pressed, in  kilogram pixel per second squared.
const MOVEMENT_FORCE: f32 = 30.;
const JUMP_SPEED: f32 = 25.;

const KEY_UP: KeyCode = KeyCode::W;
const KEY_DOWN: KeyCode = KeyCode::S;
const KEY_LEFT: KeyCode = KeyCode::A;
const KEY_RIGHT: KeyCode = KeyCode::D;

pub struct BallMovement;

#[derive(Event)]
struct MovementKeyPressed {
    direction: Vec2,
}

impl Plugin for BallMovement {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementKeyPressed>().add_systems(
            Update,
            (
                set_direction,
                move_ball,
                jump,
            )
                .chain(),
        );
    }
}

fn set_direction(k_in: Res<Input<KeyCode>>, mut writer: EventWriter<MovementKeyPressed>) {
    // TODO better solution
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
    if direction != Vec2::ZERO && !direction.is_normalized() {
        direction = direction.normalize();
    }
    writer.send(MovementKeyPressed { direction })
}

fn move_ball(
    mut query: Query<&mut ExternalForce, With<Ball>>,
    mut reader: EventReader<MovementKeyPressed>,
) {
    let mut force = query.single_mut();
    for event in reader.iter() {
        force.force = MOVEMENT_FORCE * event.direction;
    }
}

fn jump(
    mut ball_query: Query<(Entity, &mut ExternalImpulse), With<Ball>>,
    wall_query: Query<Entity, With<Wall>>,
    k_in: Res<Input<KeyCode>>,
    ctx: Res<RapierContext>,
) {
    let (ball, mut ball_imp) = ball_query.single_mut();
    let wall = wall_query.single();
    if let Some(contact_pair) = ctx.contact_pair(ball, wall) {
        if k_in.pressed(KEY_UP) && contact_pair.has_any_active_contacts() {
            ball_imp.impulse = Vec2::Y * JUMP_SPEED;
        }
    }
}
