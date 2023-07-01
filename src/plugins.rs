/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin, window::WindowResolution};
use bevy_asset_loader::prelude::*;
use bevy_egui::{
  //egui,
  //EguiContexts, 
  EguiPlugin
};
use crate::{
  core::{
    physics::CraftPhysics3CharacterDPlugin, ui::{menu::create_player::CreatePlayerPlugin, hud::hotbar::HUDHotBarPlugin, loading_asset::LoadingAssetUIPlugin}, api::{AppState, CameraState, NetworkState}, entity::{creature::player::{player_movement02, create_entity_player, player_movement01}, prefab::set_up_test}, asset::LoadingAssetPlugin
  }, systems::{load_gltf_test01}, menu::MainMenuPlugin
};
#[allow(unused_imports)]
use crate::{
  core::{
    ui::{
      network::NetworkMenuPlugin
    }, 
    data::CraftBaseDataPlugin, 
    world::prefab::WorldBasicPlugin, 
    entity::creature::player::CraftPlayerPlugin
  }, 
  systems::spawn_camera3d,
};
//lib craft
#[allow(unused_imports)]

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

pub struct BaseCraftPlugin;
impl Plugin for BaseCraftPlugin{
  fn build(&self, app: &mut App){
    app.add_state::<AppState>(); // state app
    app.add_state::<CameraState>(); // state camera mode
    app.add_state::<NetworkState>(); // state network mode
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        //width: WIDTH,
        //height: HEIGHT,
        resolution: WindowResolution::new(WIDTH, HEIGHT).with_scale_factor_override(1.0),
        title: "Bevy Game Test".to_string(),
        resizable: false,
        ..default()
      }),
      ..default()
    }));
    app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    //app.add_plugin(EguiPlugin); // menu 
    app.add_plugin(CraftBaseDataPlugin); // loading player data base
    
    // https://bevy-cheatbook.github.io/features/camera.html
    // for ui set up for camera need for render
    //app.add_startup_system(spawn_camera2d); // need this for bevy ui to render
    app.add_startup_system(spawn_camera3d); // 
    //note it need one camera at the time else log error on multiple camera active.

    //app.add_plugin(SplashScreenPlugin); // Splash Screen //nope need rework
    //app.add_plugin(LoadingAssetUIPlugin); // ui loading
    app.add_plugin(LoadingAssetPlugin); // loading call
    app.add_plugin(MainMenuPlugin); // main menu
    //app.add_plugin(WaterMarkPlugin); // water mark //testing
    //app.add_plugin(CreatePlayerPlugin); // 
    app.add_plugin(WorldBasicPlugin); // simple scene test
    //app.add_plugin(NetworkMenuPlugin); // 
    //app.add_plugin(CraftEventPlugin); // event testing...

    //app.add_plugin(PlayerPlugin);//conflict camera?
    
    //app.add_plugin(CraftPlayerPlugin); // event testing...
    app.add_plugin(CraftPhysics3CharacterDPlugin); // event testing...



  }
}


//===============================================
// MAIN?
//===============================================
pub struct DefaultCraftPlugin;

impl Plugin for DefaultCraftPlugin{//main entry point still in testing...
  fn build(&self, app: &mut App){
    app.add_state::<AppState>(); // state app
    app.add_state::<CameraState>(); // state camera mode
    app.add_state::<NetworkState>(); // state network mode
    //app.add_plugin(EguiPlugin);// menu 
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        //width: WIDTH,
        //height: HEIGHT,
        resolution: WindowResolution::new(WIDTH, HEIGHT).with_scale_factor_override(1.0),
        title: "Bevy Game Test".to_string(),
        resizable: false,
        ..default()
      }),
      ..default()
    }));
    app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    //app.add_state::<AppState>(); // state app
    
    app.add_plugin(CraftBaseDataPlugin); // loading player data base
    
    // https://bevy-cheatbook.github.io/features/camera.html
    // for ui set up for camera need for render
    //app.add_startup_system(spawn_camera2d); // need this for bevy ui to render
    app.add_startup_system(spawn_camera3d); // 
    //note it need one camera at the time else log error on multiple camera active.

    //app.add_plugin(SplashScreenPlugin); // Splash Screen //nope need rework
    app.add_plugin(LoadingAssetUIPlugin); // ui loading
    app.add_plugin(LoadingAssetPlugin); // loading call
    app.add_plugin(MainMenuPlugin); // main menu
    //app.add_plugin(WaterMarkPlugin); // water mark //testing
    app.add_plugin(CreatePlayerPlugin); // 
    //app.add_plugin(WorldBasicPlugin); // 
    //app.add_plugin(NetworkMenuPlugin); // 
    //app.add_plugin(CraftEventPlugin); // event testing...

    //app.add_plugin(PlayerPlugin);//conflict camera?
    
    //app.add_plugin(CraftPlayerPlugin); // event testing...

    app.add_plugin(HUDHotBarPlugin); // event testing...
    app.add_plugin(CraftPhysics3CharacterDPlugin); // event testing...

    //check for state
    //app.add_startup_system(check_states); //
    //test
    //app.add_system(ui_example_system);
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
    //app.add_startup_system(check_camera_state); //

    app.add_startup_system(check_app_states); //

    app.add_startup_system(set_up_test);
    app.add_startup_system(load_gltf_test01);
    //test
    //app.add_system(ui_example_system);
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
    //app.add_system(ui_example_system);
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
    //app.add_system(ui_example_system);
  }
}

fn set_network_menu(
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  app_state_next_state.set(AppState::NETWORK);
}

pub struct LoadingCraftPlayerPlugin;

impl Plugin for LoadingCraftPlayerPlugin{
  fn build(&self, app: &mut App){
    app.add_plugin(EguiPlugin);//menu egui
    app.add_state::<AppState>();//state app
    app.add_state::<CameraState>();// state camera mode
    //app.add_plugin(PlayerPlugin);

    app.add_plugin(CraftPlayerPlugin); // event testing...

    //test for state
    //app.add_startup_system(set_network_menu);

    //app.add_startup_system(create_entity_player);
    //app.add_system(player_movement01); //
    //app.add_system(player_movement02); //
    
    //test
    //app.add_system(ui_example_system);
  }
}