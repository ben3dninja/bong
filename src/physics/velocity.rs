use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Component, Default)]
pub struct Velocity(Vec3);

impl Deref for Velocity {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
