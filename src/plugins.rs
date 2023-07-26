/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::{
  prelude::*, 
  //diagnostic::FrameTimeDiagnosticsPlugin, 
  window::{WindowResolution, PresentMode}
};

//use bevy_egui::{
  //egui,
  //EguiContexts, 
  //EguiPlugin
//};

use crate::core::{systems::spawn_camera3d, ui::player::display_name::DisplayPlayerNameTextPlugin, raycast::CraftRayCastPlugin};
#[allow(unused_imports)]
use crate::core::{
    api::{
      AppState, 
      CameraState, 
      NetworkState
    }, 
    window::set_window_icon, 
    data::CraftBaseDataPlugin, 
    asset::LoadingAssetPlugin, 
    ui::{
      menu::{
        main::MainMenuPlugin, 
        options::OptionsPlugin, 
        network::NetworkMenuPlugin, 
        create_player::CreatePlayerPlugin
      }, 
      hud::hotbar::HUDHotBarPlugin, watermark::WaterMarkPlugin, splashscreen::SplashScreenPlugin, loading::LoadingAssetUIPlugin
    }, 
    event::CraftEventPlugin, 
    physics::CraftPhysics3DPlugin, 
    world::BaseWorldPlugin, 
    entity::creature::player::CraftPlayerPlugin
  };

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

pub struct BaseCraftPlugin;
//testing need to bare minimal
// for console, headless server?
impl Plugin for BaseCraftPlugin{

  fn build(&self, app: &mut App){
    app.add_state::<AppState>(); // state app
    app.add_state::<CameraState>(); // state camera mode
    app.add_state::<NetworkState>(); // state network mode
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        resolution: WindowResolution::new(WIDTH, HEIGHT).with_scale_factor_override(1.0),
        title: "Bevy Engine dhcraftrs".to_string(),
        resizable: false,
        present_mode: PresentMode::AutoNoVsync,
        ..default()
      }),
      ..default()
    }));
    app.add_systems(Startup, set_window_icon);
    //app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    //app.add_plugins(EguiPlugin); // menu 
    app.add_plugins(CraftBaseDataPlugin); // loading player data base
    
    // https://bevy-cheatbook.github.io/features/camera.html
    // for ui set up for camera need for render
    // app.add_systems(Startup,spawn_camera2d); // need this for bevy ui to render
    app.add_systems(Startup, spawn_camera3d); // 
    //note it need one camera at the time else log error on multiple camera active.

    //app.add_plugins(SplashScreenPlugin); // Splash Screen //nope need rework
    //app.add_plugins(LoadingAssetUIPlugin); // ui loading
    app.add_plugins(LoadingAssetPlugin); // loading call
    app.add_plugins(MainMenuPlugin); // main menu
    app.add_plugins(OptionsPlugin); // menu
    //app.add_plugins(WaterMarkPlugin); // water mark //testing
    //app.add_plugins(CreatePlayerPlugin); // 
    //app.add_plugins(WorldBasicPlugin); // simple scene test
    app.add_plugins(NetworkMenuPlugin); // 
    app.add_plugins(CraftEventPlugin); // event testing...

    //app.add_plugins(PlayerPlugin);//conflict camera?
    
    //app.add_plugins(CraftPlayerPlugin); // testing...
    app.add_plugins(CraftPhysics3DPlugin); // testing...
    app.add_plugins(DisplayPlayerNameTextPlugin); // testing...

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
        resolution: WindowResolution::new(WIDTH, HEIGHT).with_scale_factor_override(1.0),
        title: "Bevy Game Test".to_string(),
        resizable: false,
        ..default()
      }),
      ..default()
    }));
    app.add_systems(Startup, set_window_icon);
    //app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(CraftBaseDataPlugin); // loading player data base
    app.add_plugins(CraftRayCastPlugin); // 
    
    // https://bevy-cheatbook.github.io/features/camera.html
    // for ui set up for camera need for render
    // need this for bevy ui to render
    //app.add_startup_system(spawn_camera2d); 
    app.add_systems(Startup,spawn_camera3d); // 
    //note it need one camera at the time else log error on multiple camera active.

    //app.add_plugin(WaterMarkPlugin); // water mark //testing
    app.add_plugins(SplashScreenPlugin); // Splash Screen //nope need rework

    app.add_plugins(LoadingAssetUIPlugin); // ui loading
    app.add_plugins(LoadingAssetPlugin); // loading call
    app.add_plugins(MainMenuPlugin); // main menu
    app.add_plugins(OptionsPlugin); // menu
    app.add_plugins(CreatePlayerPlugin); // 

    //app.add_plugins(CraftRayCastPlugin); // 
    //app.add_plugins(WorldBasicPlugin); // 
    app.add_plugins(NetworkMenuPlugin); // 
    app.add_plugins(CraftEventPlugin); // Events

    //app.add_plugins(PlayerPlugin);//conflict camera?
    //app.add_plugins(CraftPlayerPlugin); //

    app.add_plugins(HUDHotBarPlugin); // set up hot bar
    app.add_plugins(CraftPhysics3DPlugin); // set up physics

    // Test scene place 
    app.add_plugins(BaseWorldPlugin); //preload entity tests
    //app.add_plugins(NetworkCraftPlugin); //
    //app.add_plugins(CraftSubAppPlugin); // testing app and sub app
    app.add_plugins(DisplayPlayerNameTextPlugin); // testing...
    
  }
}

pub struct LoadingCraftPlayerPlugin;

impl Plugin for LoadingCraftPlayerPlugin{
  fn build(&self, app: &mut App){
    //app.add_plugin(EguiPlugin); //menu egui
    app.add_state::<AppState>(); //state app
    app.add_state::<CameraState>(); // state camera mode
    //app.add_plugin(PlayerPlugin);

    app.add_plugins(CraftPlayerPlugin); // 

    //test for state
    //app.add_startup_system(set_network_menu);

    //app.add_startup_system(create_entity_player);
    //app.add_system(player_movement01); //
    //app.add_system(player_movement02); //
    
    //test
    //app.add_system(ui_example_system);
  }
}