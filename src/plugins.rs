/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//===============================================
// base set up
//===============================================

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

//lib craft
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
    app.add_state::<AppState>();//state app
    app.add_state::<CameraState>();// state camera mode
    app.add_plugin(MainMenuPlugin);//menu setup
    //loading assets state
    app.add_loading_state(
      LoadingState::new(AppState::AssetLoading)
        .continue_to_state(AppState::MainMenu)
    );
    //loading assets
    app.add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading);
    //assets do something
    app.add_system(use_my_assets.in_schedule(OnEnter(AppState::MainMenu)));
    //set up camera 2d for render UI
    app.add_startup_system(spawn_camera);//need this for bevy ui to render
    //check for state
    app.add_startup_system(check_states);//
  }
}