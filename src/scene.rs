use bevy::prelude::*;

const WALL_COLOR: Color = Color::BLUE;
pub const WALL_POSITION: Vec3 = Vec3::new(0., -200., 0.);
pub const WALL_SCALE: Vec3 = Vec3::new(500., 50., 1.);

#[derive(Component)]
pub struct Wall;

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_scene);
    }
}

fn create_scene(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform {
                translation: WALL_POSITION,
                scale: WALL_SCALE,
                ..default()
            },
            ..default()
        },
        Wall,
    ));
}
