use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerState>();
    }
}

enum PlayerState {
    Idle,
    Jumping,
    Falling,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idle
    }
}
