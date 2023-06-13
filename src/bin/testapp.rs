use bevy::{
  prelude::*, 
  //winit::WinitSettings,
  //tasks::IoTaskPool,
  //utils::Duration
};

use bevy_asset_loader::prelude::*;

use dhcraftrs::menu::MainMenuPlugin;
use dhcraftrs::AppState;
use dhcraftrs::systems::spawn_camera;
use dhcraftrs::assets::MyAssets;
use dhcraftrs::assets::use_my_assets;


fn main() {

  println!("Test App!");

  App::new()
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    .add_plugin(MainMenuPlugin)
    .add_loading_state(
      LoadingState::new(AppState::AssetLoading)
          .continue_to_state(AppState::MainMenu)
    )
    .add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading)
    .add_system(use_my_assets.in_schedule(OnEnter(AppState::MainMenu)))

    .add_startup_system(spawn_camera)//need this for bevy ui to render
    .run();

}