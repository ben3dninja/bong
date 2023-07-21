use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use bevy_rapier2d::prelude::*;

use crate::input::PlayerControlled;

use super::super::Ball;

const HEAVY_CODE: KeyCode = KeyCode::Space;
const HEAVINESS_FACTOR: f32 = 0.1;
const HEAVYNESS_DURATION: Duration = Duration::new(5, 0);

pub struct HeavyPlugin;

impl Plugin for HeavyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (detect_heavier, tick_timers, update_mass, change_color).chain(),
        );
    }
}

fn detect_heavier(k_in: Res<Input<KeyCode>>, mut query: Query<&mut Ball, With<PlayerControlled>>) {
    query.single_mut().is_heavy = k_in.pressed(HEAVY_CODE);
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
                (HEAVYNESS_DURATION.saturating_sub(ball.heavy_timer.elapsed())).as_secs_f32()
                    * HEAVINESS_FACTOR,
            )
        } else {
            AdditionalMassProperties::default()
        }
    }
}

fn change_color(query: Query<&Ball>, mut assets: ResMut<Assets<ColorMaterial>>) {
    for ball in query.iter() {
        let color = if let Some(original_material) = assets.get(&ball.original_material) {
            if ball.is_heavy {
                apply_saturation_ratio(
                    original_material.color,
                    ball.heavy_timer.elapsed_secs() / HEAVYNESS_DURATION.as_secs_f32(),
                )
            } else {
                original_material.color
            }
        } else {
            Color::default()
        };
        if let Some(material) = assets.get_mut(&ball.material) {
            material.color = color;
        }
    }
}

fn backwards_tick(timer: &mut Stopwatch, delta: Duration) {
    timer.set_elapsed(timer.elapsed().saturating_sub(delta));
}

fn apply_saturation_ratio(color: Color, ratio: f32) -> Color {
    if let Color::Hsla {
        hue,
        saturation,
        lightness,
        alpha,
    } = color.as_hsla()
    {
        Color::Hsla {
            hue,
            saturation: saturation * ratio,
            lightness,
            alpha,
        }
    } else {
        // TODO not elegant
        panic!()
    }
}
