use bevy::prelude::*;

pub use self::mass::Mass;
pub use self::velocity::Velocity;

mod mass;
mod velocity;

/// Gravitational intensity (g) in pixels per second squared
const GRAVITATIONAL_INTENSITY: f32 = 600.;

pub struct Physics;

impl Plugin for Physics {
    fn build(&self, app: &mut App) {
        app.add_system(update_translation)
            .add_system(gravity.before(update_translation));
    }
}

fn update_translation(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        // v = dr/dt -> dr = v dt
        transform.translation += **velocity * time.delta_seconds();
    }
}

#[derive(Component)]
pub struct Gravity;

fn gravity(mut query: Query<&mut Velocity, With<Gravity>>, time: Res<Time>) {
    for mut velocity in query.iter_mut() {
        // m dv/dt = -mg -> dv = -g dt
        velocity.y -= GRAVITATIONAL_INTENSITY * time.delta_seconds();
    }
}
