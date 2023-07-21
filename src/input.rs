use bevy::prelude::*;
use serde::{Deserialize, Serialize};

mod direction;

pub use direction::*;

use crate::ball::Ball;

const KEY_UP: KeyCode = KeyCode::W;
const KEY_DOWN: KeyCode = KeyCode::S;
const KEY_LEFT: KeyCode = KeyCode::A;
const KEY_RIGHT: KeyCode = KeyCode::D;

pub const KEY_HEAVY: KeyCode = KeyCode::Space;

const KEYS: [KeyCode; 5] = [KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_HEAVY];

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput::default())
            .add_event::<PlayerInputEvent>()
            .add_systems(Update, (record_input, update_player_ball).chain());
    }
}

#[derive(Default, Serialize, Deserialize, Resource)]
pub struct PlayerInput {
    pub direction: DirectionVector,
    pub is_heavy: bool,
}

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Event)]
struct PlayerInputEvent;

fn record_input(
    mut input: ResMut<PlayerInput>,
    mut writer: EventWriter<PlayerInputEvent>,
    k_in: Res<Input<KeyCode>>,
) {
    if k_in.any_just_pressed(KEYS) || k_in.any_just_released(KEYS) {
        let mut direction = Vec2::ZERO;
        for key in k_in.get_pressed() {
            direction += match *key {
                KEY_UP => Vec2::Y,
                KEY_DOWN => Vec2::NEG_Y,
                KEY_RIGHT => Vec2::X,
                KEY_LEFT => Vec2::NEG_X,
                _ => Vec2::ZERO,
            }
        }
        input.direction = DirectionVector::new_normalize(direction);
        input.is_heavy = k_in.pressed(KEY_HEAVY);
        writer.send(PlayerInputEvent);
    }
}

fn update_player_ball(
    mut query: Query<(&mut DirectionVector, &mut Ball), With<PlayerControlled>>,
    mut reader: EventReader<PlayerInputEvent>,
    input: Res<PlayerInput>,
) {
    for _ in reader.iter() {
        let (mut direction, mut ball) = query.single_mut();
        *direction = input.direction;
        ball.is_heavy = input.is_heavy;
    }
}
