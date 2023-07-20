use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use bevy_rapier2d::prelude::*;

use crate::ball::BALL_COLOR;

use super::super::Ball;

const HEAVY_CODE: KeyCode = KeyCode::Space;
const HEAVINESS_FACTOR: f32 = 0.1;
const HEAVYNESS_DURATION: Duration = Duration::new(5, 0);

pub struct HeavyPlugin;

impl Plugin for HeavyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HeavyTimer(Stopwatch::new()))
            .add_systems(Update, (tick_timer, update_mass, change_color).chain());
    }
}

#[derive(Resource)]
struct HeavyTimer(Stopwatch);

fn tick_timer(k_in: Res<Input<KeyCode>>, mut timer: ResMut<HeavyTimer>, time: Res<Time>) {
    if k_in.pressed(HEAVY_CODE) {
        timer.0.tick(time.delta());
    } else if !timer.0.elapsed().is_zero() {
        backwards_tick(&mut timer.0, time.delta())
    }
}

fn update_mass(
    mut query: Query<&mut AdditionalMassProperties>,
    timer: Res<HeavyTimer>,
    k_in: Res<Input<KeyCode>>,
) {
    let mut additional_mass = query.single_mut();
    *additional_mass = if k_in.pressed(HEAVY_CODE) {
        AdditionalMassProperties::Mass(
            (HEAVYNESS_DURATION.saturating_sub(timer.0.elapsed())).as_secs_f32() * HEAVINESS_FACTOR,
        )
    } else {
        AdditionalMassProperties::default()
    }
}

fn change_color(
    mut query: Query<&Ball>,
    timer: Res<HeavyTimer>,
    k_in: Res<Input<KeyCode>>,
    mut assets: ResMut<Assets<ColorMaterial>>,
) {
    let ball = query.single_mut();
    if let Some(material) = assets.get_mut(&ball.material_handle) {
        material.color = if k_in.pressed(HEAVY_CODE) {
            apply_saturation_ratio(
                BALL_COLOR,
                timer.0.elapsed_secs() / HEAVYNESS_DURATION.as_secs_f32(),
            )
        } else {
            BALL_COLOR
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
