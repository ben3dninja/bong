use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{Heavy, Lobby, BALL_RADIUS, HEAVINESS_DURATION};

const BALL_COLOR: Color = Color::rgb(0.0, 0.38, 0.39);

#[derive(Component)]
pub(super) struct BallDisplay {
    material: Handle<ColorMaterial>,
    original_material: Handle<ColorMaterial>,
}

pub(super) fn display_balls(
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for data in lobby.players.values_mut() {
        let material = materials.add(BALL_COLOR.into());
        let mesh = meshes.add(shape::Circle::new(BALL_RADIUS).into()).into();
        let original_material = materials.add(BALL_COLOR.into());
        // TODO unwraps
        let entity = commands
            .get_entity(data.entity.unwrap())
            .unwrap()
            .insert((
                BallDisplay {
                    material: material.clone(),
                    original_material,
                },
                Heavy::default(),
                MaterialMesh2dBundle {
                    mesh,
                    material,
                    transform: Transform::from_translation(data.spawning_location),
                    ..default()
                },
            ))
            .id();
        data.entity = Some(entity);
    }
}

pub(super) fn update_ball_colors(
    query: Query<(&BallDisplay, &Heavy)>,
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
