use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use bevy_rapier2d::prelude::*;

use crate::{HeavinessReceivedEvent, Heavy, Lobby, Processing, HEAVINESS_DURATION};

pub const HEAVINESS_FACTOR: f32 = 0.1;

pub struct HeavyPlugin;

impl Plugin for HeavyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (update_heavy, tick_timers, update_mass)
                .in_set(Processing)
                .chain(),
        );
    }
}

fn update_heavy(
    mut query: Query<&mut Heavy>,
    lobby: Res<Lobby>,
    mut event_reader: EventReader<HeavinessReceivedEvent>,
) {
    for HeavinessReceivedEvent { origin, heaviness } in event_reader.iter() {
        // TODO : unwraps
        dbg!("Heaviness received for entity", origin);
        let mut heavy = query
            .get_mut(lobby.players.get(origin).unwrap().entity.unwrap())
            .unwrap();
        heavy.heaviness = *heaviness;
    }
}

fn tick_timers(mut query: Query<&mut Heavy>, time: Res<Time>) {
    for mut heavy in query.iter_mut() {
        if heavy.heaviness {
            heavy.heavy_timer.tick(time.delta());
        } else if !heavy.heavy_timer.elapsed().is_zero() {
            backwards_tick(&mut heavy.heavy_timer, time.delta())
        }
    }
}

fn update_mass(mut query: Query<(&Heavy, &mut AdditionalMassProperties)>) {
    for (heavy, mut additional_mass) in query.iter_mut() {
        *additional_mass = if heavy.heaviness {
            AdditionalMassProperties::Mass(
                (HEAVINESS_DURATION.saturating_sub(heavy.heavy_timer.elapsed())).as_secs_f32()
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
