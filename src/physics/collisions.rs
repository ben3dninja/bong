use bevy::prelude::*;

#[derive(Component)]
pub struct BounceCollider {
    pub material: ColliderMaterial,
}

enum ColliderMaterial {
    Dampen,
    Elastic,
}

#[derive(Component)]
pub struct Static;

#[derive(Component)]
pub struct Deadly;

struct CollisionEvent;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>().add_systems(Update, (detect_collision));
    }
}

fn detect_collision(ball_query: Query<(&Velocity, &Transform), With<Ball>>, writer: EventWriter<CollisionEvent>)
