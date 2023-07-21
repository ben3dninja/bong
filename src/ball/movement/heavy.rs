use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use bevy_rapier2d::prelude::*;

use super::super::Ball;

pub const HEAVINESS_FACTOR: f32 = 0.1;
pub const HEAVINESS_DURATION: Duration = Duration::new(5, 0);

pub struct HeavyPlugin;

impl Plugin for HeavyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tick_timers, update_mass).chain());
    }
}

fn tick_timers(mut query: Query<&mut Ball>, time: Res<Time>) {
    for mut ball in query.iter_mut() {
        if ball.is_heavy {
            ball.heavy_timer.tick(time.delta());
        } else if !ball.heavy_timer.elapsed().is_zero() {
            backwards_tick(&mut ball.heavy_timer, time.delta())
        }
    }
}

fn update_mass(mut query: Query<(&Ball, &mut AdditionalMassProperties)>) {
    for (ball, mut additional_mass) in query.iter_mut() {
        *additional_mass = if ball.is_heavy {
            AdditionalMassProperties::Mass(
                (HEAVINESS_DURATION.saturating_sub(ball.heavy_timer.elapsed())).as_secs_f32()
                    * HEAVINESS_FACTOR,
            )
        } else {
            AdditionalMassProperties::default()
        }
    }
}

fn backwards_tick(timer: &mut Stopwatch, delta: Duration) {
    timer.set_elapsed(timer.elapsed().saturating_sub(delta));
}
