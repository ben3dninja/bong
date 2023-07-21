use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::ball::{
    movement::heavier::HEAVINESS_DURATION, Ball, BALL_RADIUS, BALL_STARTING_TRANSFORM,
};

const BALL_COLOR: Color = Color::rgb(0.0, 0.38, 0.39);

#[derive(Resource)]
pub(super) struct BallMesh(Mesh2dHandle);

#[derive(Component)]
pub(super) struct BallDisplay {
    material: Handle<ColorMaterial>,
    original_material: Handle<ColorMaterial>,
}

pub(super) fn create_ball_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(BallMesh(
        meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
    ));
}

pub(super) fn display_balls(
    mut commands: Commands,
    query: Query<Entity, With<Ball>>,
    mesh_handle: Res<BallMesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in query.iter() {
        let material = materials.add(BALL_COLOR.into());
        let original_material = materials.add(BALL_COLOR.into());
        commands.entity(entity).insert((
            MaterialMesh2dBundle {
                mesh: mesh_handle.0.clone(),
                material: material.clone(),
                transform: BALL_STARTING_TRANSFORM,
                ..default()
            },
            BallDisplay {
                material,
                original_material,
            },
        ));
    }
}

pub(super) fn change_color(
    query: Query<(&BallDisplay, &Ball)>,
    mut assets: ResMut<Assets<ColorMaterial>>,
) {
    for (display, ball) in query.iter() {
        let color = if let Some(original_material) = assets.get(&display.original_material) {
            if ball.is_heavy {
                apply_saturation_ratio(
                    original_material.color,
                    ball.heavy_timer.elapsed_secs() / HEAVINESS_DURATION.as_secs_f32(),
                )
            } else {
                original_material.color
            }
        } else {
            Color::default()
        };
        if let Some(material) = assets.get_mut(&display.material) {
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
