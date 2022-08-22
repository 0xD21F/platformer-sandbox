use bevy::{prelude::*, render::render_resource::Texture};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    debug::DebugPlugin, level::LevelPlugin, movement::MovementPlugin, player::PlayerPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Playing)
                .with_collection::<GameAssets>(),
        )
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(MovementPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..default()
        })
        .insert_resource(LevelSelection::Identifier("Start".to_owned()))
        .add_enter_system(GameState::Playing, setup);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    Playing,
}

// Spawn our camera and LDTK World
fn setup(mut commands: Commands, images: Res<GameAssets>) {
    let camera = Camera2dBundle::default();

    commands.spawn_bundle(camera);

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: images.map.clone(),
        ..default()
    });
}

#[derive(AssetCollection)]
struct GameAssets {
    #[asset(path = "sandbox.ldtk")]
    map: Handle<LdtkAsset>,
    #[asset(path = "player/placeholder.png")]
    playersprite: Handle<Image>,
}
