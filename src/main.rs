use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use debug::DebugPlugin;
use drif_platformer_sandbox::GameState;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

mod debug;

fn main() {
    let mut app = App::new();

    app.add_loopless_state(GameState::AssetLoading)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Playing)
                .with_collection::<GameAssets>(),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(ProgressPlugin::new(GameState::AssetLoading))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LdtkPlugin)
        .add_plugin(DebugPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..default()
        })
        .insert_resource(LevelSelection::Index(0))
        .add_enter_system(GameState::Playing, setup)
        .add_system_to_stage(CoreStage::PostUpdate, print_progress)
        .run();
}

#[derive(AssetCollection)]
struct GameAssets {
    #[asset(path = "sandbox.ldtk")]
    map: Handle<LdtkAsset>,
}

fn setup(mut commands: Commands, images: Res<GameAssets>) {
    let camera = Camera2dBundle::default();

    commands.spawn_bundle(camera);

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: images.map.clone(),
        ..default()
    });
}

fn print_progress(progress: Option<Res<ProgressCounter>>) {
    if let Some(progress) = progress {
        info!("Current progress: {:?}", progress.progress());
    }
}
