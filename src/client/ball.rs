use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{GameState, Heavy, Lobby, BALL_RADIUS, HEAVINESS_DURATION};

const BALL_COLOR: Color = Color::rgb(0.0, 0.38, 0.39);

pub(super) struct BallsPlugin;

impl Plugin for BallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_ball_mesh)
            .add_systems(OnEnter(GameState::InGame), spawn_balls)
            .add_systems(
                Update,
                update_ball_colors
                    .after(super::receive_entities_in_game)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), despawn_balls);
    }
}

#[derive(Component)]
pub(super) struct Ball {
    material: Handle<ColorMaterial>,
    original_material: Handle<ColorMaterial>,
}

pub(super) fn spawn_balls(
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mesh_handle: Res<BallMesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for data in lobby.players.values_mut() {
        let material = materials.add(BALL_COLOR.into());
        let original_material = materials.add(BALL_COLOR.into());
        let entity = commands
            .spawn((
                Ball {
                    material: material.clone(),
                    original_material,
                },
                Heavy::default(),
                MaterialMesh2dBundle {
                    mesh: mesh_handle.0.clone(),
                    material,
                    transform: Transform::from_translation(data.spawning_location),
                    ..default()
                },
            ))
            .id();
        data.entity = Some(entity);
    }
}

pub(super) fn despawn_balls(mut commands: Commands, balls: Query<Entity, With<Ball>>) {
    for ball in balls.iter() {
        commands.get_entity(ball).unwrap().despawn_recursive();
    }
}

#[derive(Resource)]
pub(super) struct BallMesh(Mesh2dHandle);

pub(super) fn create_ball_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(BallMesh(
        meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
    ));
}

pub(super) fn update_ball_colors(
    query: Query<(&Ball, &Heavy)>,
    mut assets: ResMut<Assets<ColorMaterial>>,
) {
    for (ball, heavy) in query.iter() {
        let color = if let Some(original_material) = assets.get(&ball.original_material) {
            if heavy.heaviness {
                apply_saturation_ratio(
                    original_material.color,
                    heavy.heavy_timer.elapsed_secs() / HEAVINESS_DURATION.as_secs_f32(),
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
