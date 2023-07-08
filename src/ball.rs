use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct BallPlugin;

const BALL_RADIUS: f32 = 30.;
const BALL_POSITION: Vec3 = Vec3::new(0., 0., 0.);
const BALL_COLOR: Color = Color::PURPLE;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_ball);
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(BALL_COLOR.into()),
            transform: Transform::from_translation(BALL_POSITION),
            ..default()
        })
        .insert(Ball);
}

#[derive(Component)]
struct Ball;
