use bevy::prelude::*;

use crate::GameState;

mod ball;
mod scene;

pub(super) struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            ball::display_balls.after(crate::ball::spawn_balls),
        )
        .add_systems(
            Update,
            ball::update_ball_colors
                .after(crate::communication::receive_player_heaviness)
                .run_if(in_state(GameState::InGame)),
        );
    }
}
