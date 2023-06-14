use bevy::{
  prelude::*, 
  //winit::WinitSettings,
  //tasks::IoTaskPool,
  //utils::Duration
};
use dhcraftrs::plugins::DefaultCraftPlugin;
//use bevy_asset_loader::prelude::*;
//use dhcraftrs::menu::MainMenuPlugin;
//use dhcraftrs::systems::check_states;
//use dhcraftrs::systems::spawn_camera;
//use dhcraftrs::assets::MyAssets;
//use dhcraftrs::systems::use_my_assets;
//use dhcraftrs::api::AppState;
//use dhcraftrs::api::CameraState;


fn main() {//Entry point

  println!("Test App!");

  App::new()
    .add_plugins(DefaultPlugins)//window scree set up
    .add_plugin(DefaultCraftPlugin) //craft set up
    //.add_state::<AppState>()
    //.add_state::<CameraState>()
    //.add_plugin(MainMenuPlugin)
    //.add_loading_state(
      //LoadingState::new(AppState::AssetLoading)
          //.continue_to_state(AppState::MainMenu)
    //)
    //.add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading)
    //.add_system(use_my_assets.in_schedule(OnEnter(AppState::MainMenu)))
    //.add_startup_system(spawn_camera)//need this for bevy ui to render
    //.add_startup_system(check_states)//need this for bevy ui to render
    .run();

}