use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Component)]
pub struct Mass(f32);

impl Deref for Mass {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Mass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Mass {
    fn default() -> Self {
        Self(1.)
    }
}
