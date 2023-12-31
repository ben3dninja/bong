use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use self::movement::BallMovement;

mod movement;

/// Ball radius, in pixels
pub const BALL_RADIUS: f32 = 20.;
/// Ball starting position, in pixels
pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0., 0., 0.);
const BALL_COLOR: Color = Color::rgb(0.0, 0.38, 0.39);

#[derive(Component)]
pub struct Ball {
    material_handle: Handle<ColorMaterial>,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_ball)
            .add_plugins(BallMovement);
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material_handle = materials.add(BALL_COLOR.into());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: material_handle.clone(),
            transform: Transform::from_translation(BALL_STARTING_POSITION),
            ..default()
        },
        Ball { material_handle },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::ball(BALL_RADIUS),
        ExternalForce::default(),
        ExternalImpulse::default(),
        Ccd::enabled(),
        Sleeping::disabled(),
        GravityScale(4.5),
        AdditionalMassProperties::default(),
    ));
}
