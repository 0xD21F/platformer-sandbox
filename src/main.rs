use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;
use leafwing_input_manager::prelude::*;
use platformer_sandbox::{actions::PlatformerAction, game::*};

fn main() {
    let mut app = App::new();

    // Load assets
    app.add_loopless_state(GameState::AssetLoading)
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(ProgressPlugin::new(GameState::AssetLoading))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InputManagerPlugin::<PlatformerAction>::default())
        .add_plugin(LdtkPlugin)
        .add_plugin(GamePlugin)
        .run();
}
