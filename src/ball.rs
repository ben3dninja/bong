use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    time::Stopwatch,
};
use bevy_rapier2d::prelude::*;

use crate::input::{DirectionVector, PlayerControlled};

use self::movement::BallMovement;

mod movement;

/// Ball radius, in pixels
pub const BALL_RADIUS: f32 = 20.;
/// Ball starting position, in pixels
pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0., 0., 0.);
const BALL_COLOR: Color = Color::rgb(0.0, 0.38, 0.39);

#[derive(Default, Component)]
pub struct Ball {
    material: Handle<ColorMaterial>,
    original_material: Handle<ColorMaterial>,
    is_heavy: bool,
    heavy_timer: Stopwatch,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, create_mesh)
            .add_systems(PostStartup, spawn_ball)
            .add_plugins(BallMovement);
    }
}

#[derive(Resource)]
struct BallMesh(Mesh2dHandle);

fn create_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(BallMesh(
        meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
    ));
}

fn spawn_ball(
    mut commands: Commands,
    mesh_handle: Res<BallMesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = materials.add(BALL_COLOR.into());
    let original_material = materials.add(BALL_COLOR.into());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle.0.clone(),
            material: material.clone(),
            transform: Transform::from_translation(BALL_STARTING_POSITION),
            ..default()
        },
        Ball {
            material,
            original_material,
            ..default()
        },
        PlayerControlled,
        DirectionVector::default(),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::ball(BALL_RADIUS),
        ExternalForce::default(),
        ExternalImpulse::default(),
        Ccd::enabled(),
        GravityScale(4.5),
        AdditionalMassProperties::default(),
        Sleeping::disabled(),
    ));
}
