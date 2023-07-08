use bevy::prelude::*;

pub use self::mass::Mass;
pub use self::velocity::Velocity;

mod mass;
mod velocity;

pub struct Physics;

impl Plugin for Physics {
    fn build(&self, app: &mut App) {
        app.add_system(update_translation);
    }
}

fn update_translation(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        // v = dr/dt -> dr = v dt
        transform.translation += **velocity * time.delta_seconds();
    }
}
