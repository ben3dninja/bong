use std::f32::consts::SQRT_2;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Serialize, Deserialize)]
pub enum InputDirection {
    #[default]
    Zero,
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

const NORM: f32 = 1. / SQRT_2;

impl Into<DirectionVector> for InputDirection {
    fn into(self) -> DirectionVector {
        use InputDirection::*;
        DirectionVector::new_normalize(match self {
            Up => Vec2::Y,
            Down => Vec2::NEG_Y,
            Right => Vec2::X,
            Left => Vec2::NEG_X,
            UpRight => (Vec2::Y + Vec2::X) * NORM,
            UpLeft => (Vec2::Y + Vec2::NEG_X) * NORM,
            DownRight => (Vec2::NEG_Y + Vec2::X) * NORM,
            DownLeft => (Vec2::NEG_Y + Vec2::NEG_X) * NORM,
            Zero => Vec2::ZERO,
        })
    }
}

#[derive(Copy, Clone, Default, Serialize, Deserialize, Component)]
pub struct DirectionVector(Vec2);

impl std::ops::Deref for DirectionVector {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DirectionVector {
    pub fn new(direction: Vec2) -> Self {
        if !direction.is_normalized() {
            panic!(
                "Direction vector not normalized: {} has norm {}",
                direction,
                direction.length()
            );
        }
        Self(direction)
    }

    pub fn new_normalize(direction: Vec2) -> Self {
        Self(direction.normalize_or_zero())
    }
}

impl Into<Vec2> for DirectionVector {
    fn into(self) -> Vec2 {
        self.0
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[should_panic]
    fn not_normalized_panics() {
        DirectionVector::new(Vec2::ONE);
    }

    #[test]
    fn normalization() {
        assert!(DirectionVector::new_normalize(Vec2::ONE).is_normalized());
    }
}
