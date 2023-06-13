//===============================================
// base set up
//===============================================

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
  api::{
    AppState,
    CameraState
  }, 
  systems::{
    spawn_camera,
    check_states, 
    use_my_assets,
  },
  assets::MyAssets, 
  menu::MainMenuPlugin
};
pub struct DefaultCraftPlugin;

impl Plugin for DefaultCraftPlugin{
  fn build(&self, app: &mut App){

    app.add_state::<AppState>();
    app.add_state::<CameraState>();
    app.add_plugin(MainMenuPlugin);
    app.add_loading_state(
      LoadingState::new(AppState::AssetLoading)
        .continue_to_state(AppState::MainMenu)
    );
    app.add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading);
    app.add_system(use_my_assets.in_schedule(OnEnter(AppState::MainMenu)));
    app.add_startup_system(spawn_camera);//need this for bevy ui to render
    app.add_startup_system(check_states);//
  }
}