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
use bevy_egui::{egui, EguiContexts, EguiPlugin};
//lib craft
use crate::{
  
  api::{
    AppState,
    CameraState
  }, 
  systems::{
    spawn_camera,
    check_states, 
    use_my_assets, load_gltf_test01, spawn_camera3d,
  },
  assets::MyAssets, 
  menu::MainMenuPlugin, 
  core::{ui::{editor::systems::layout::ui_example_system}, entity::{prefab::{build_cube, set_up_test}, creature::player::{player_movement, create_entity_player, player_movement01, player_movement02}}}
};
pub struct EntryMenuPlugin;

impl Plugin for EntryMenuPlugin{
  fn build(&self, app: &mut App){
    //app.add_plugin(EguiPlugin);//menu 
    app.add_state::<AppState>();//state app
    app.add_state::<CameraState>();// state camera mode
    app.add_plugin(MainMenuPlugin);//menu
    app.add_startup_system(spawn_camera);//need this for bevy ui to render
    //loading assets state
    app.add_loading_state(
      LoadingState::new(AppState::AssetLoading)
        .continue_to_state(AppState::MainMenu)
    );
    //loading assets
    app.add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading);
    //assets do something
    app.add_system(use_my_assets.in_schedule(OnEnter(AppState::MainMenu)));
    //check for state
    app.add_startup_system(check_states); //

    //test
    //app.add_system(ui_example_system);
  }
}

pub struct DefaultCraftPlugin;

impl Plugin for DefaultCraftPlugin{
  fn build(&self, app: &mut App){
    app.add_plugin(EguiPlugin);//menu 
    app.add_state::<AppState>();//state app
    app.add_state::<CameraState>();// state camera mode
    app.add_plugin(MainMenuPlugin);//menu
    app.add_startup_system(spawn_camera);//need this for bevy ui to render
    //loading assets state
    app.add_loading_state(
      LoadingState::new(AppState::AssetLoading)
        .continue_to_state(AppState::MainMenu)
    );
    //loading assets
    app.add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading);
    //assets do something
    app.add_system(use_my_assets.in_schedule(OnEnter(AppState::MainMenu)));
    //check for state
    app.add_startup_system(check_states); //

    //test
    app.add_system(ui_example_system);
  }
}

pub struct Test01CraftPlugin;

impl Plugin for Test01CraftPlugin{
  fn build(&self, app: &mut App){
    app.add_plugin(EguiPlugin);//menu 
    app.add_state::<AppState>();//state app
    app.add_state::<CameraState>();// state camera mode

    app.add_startup_system(set_game_state); //
    app.add_plugin(MainMenuPlugin);//menu
    //app.add_startup_system(spawn_camera);//need this for bevy ui to render
    //loading assets state
    //app.add_loading_state(
      //LoadingState::new(AppState::AssetLoading)
        //.continue_to_state(AppState::MainMenu)
    //);
    //loading assets
    //app.add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading);
    //assets do something
    //app.add_system(use_my_assets.in_schedule(OnEnter(AppState::MainMenu)));
    //check for state
    app.add_startup_system(check_states); //

    app.add_startup_system(check_app_states); //

    app.add_startup_system(set_up_test);
    app.add_startup_system(load_gltf_test01);
    //test
    app.add_system(ui_example_system);
  }
}

fn set_game_state(
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  app_state_next_state.set(AppState::InGame);
}

fn check_app_states(
  app_state_next_state:ResMut<NextState<AppState>>,
){
  println!("app_state_next_state: {:?}", app_state_next_state.0);
}

//test simple character movement 
pub struct Test02CraftPlugin;
use bevy_flycam::PlayerPlugin;

impl Plugin for Test02CraftPlugin{
  fn build(&self, app: &mut App){
    app.add_plugin(EguiPlugin);//menu egui
    app.add_state::<AppState>();//state app
    app.add_state::<CameraState>();// state camera mode

    //test for state
    //app.add_startup_system(set_game_state); //
    //app.add_startup_system(check_states); //
    //app.add_startup_system(check_app_states); //
    //app.add_startup_system(set_up_test); //
    //player setup, config and logic
    //app.add_startup_system(spawn_camera3d);
    app.add_plugin(PlayerPlugin);
    app.add_startup_system(create_entity_player);
    app.add_system(player_movement01); //
    
    //test
    app.add_system(ui_example_system);
  }
}


pub struct Test03CraftPlugin;

impl Plugin for Test03CraftPlugin{
  fn build(&self, app: &mut App){
    app.add_plugin(EguiPlugin);//menu egui
    app.add_state::<AppState>();//state app
    app.add_state::<CameraState>();// state camera mode
    app.add_plugin(PlayerPlugin);

    //test for state
    app.add_startup_system(set_network_menu);

    app.add_startup_system(create_entity_player);
    //app.add_system(player_movement01); //
    app.add_system(player_movement02); //
    
    //test
    app.add_system(ui_example_system);
  }
}

fn set_network_menu(
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  app_state_next_state.set(AppState::NETWORK);
}

pub struct LoadingTextCraftPlugin;

impl Plugin for LoadingTextCraftPlugin{
  fn build(&self, app: &mut App){
    app.add_plugin(EguiPlugin);//menu egui
    app.add_state::<AppState>();//state app
    app.add_state::<CameraState>();// state camera mode
    app.add_plugin(PlayerPlugin);

    //test for state
    //app.add_startup_system(set_network_menu);

    //app.add_startup_system(create_entity_player);
    //app.add_system(player_movement01); //
    //app.add_system(player_movement02); //
    
    //test
    app.add_system(ui_example_system);
  }
}