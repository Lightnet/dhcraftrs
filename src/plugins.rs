/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::{
  prelude::*, 
  //diagnostic::FrameTimeDiagnosticsPlugin, 
  window::WindowResolution
};

use bevy_egui::{
  //egui,
  //EguiContexts, 
  EguiPlugin
};
use crate::core::event::CraftEventPlugin;
use crate::core::network::NetworkCraftPlugin;
use crate::core::ui::menu::create_player::CreatePlayerPlugin;
use crate::core::ui::menu::network::NetworkMenuPlugin;
use crate::core::ui::menu::options::OptionsPlugin;
#[allow(unused_imports)]
use crate::core::{ui::hud::hotbar::HUDHotBarPlugin, physics::CraftPhysics3DPlugin, subapp::CraftSubAppPlugin};
#[allow(unused_imports)]
use crate::{
  core::{
    ui::{
      menu::{ main::MainMenuPlugin}, 
    }, 
    api::{
      AppState, 
      CameraState, 
      NetworkState
    }, 
    asset::LoadingAssetPlugin, window::set_window_icon, raycast::CraftRayCastPlugin, world::BaseWorldPlugin
  }, 
};
#[allow(unused_imports)]
use crate::{
  core::{
    data::CraftBaseDataPlugin, 
    world::prefab::WorldBasicPlugin, 
    entity::creature::player::CraftPlayerPlugin
  }, 
  systems::spawn_camera3d,
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
        //width: WIDTH,
        //height: HEIGHT,
        resolution: WindowResolution::new(WIDTH, HEIGHT).with_scale_factor_override(1.0),
        title: "Bevy Engine dhcraftrs".to_string(),
        resizable: false,
        ..default()
      }),
      ..default()
    }));
    app.add_startup_system(set_window_icon);
    //app.add_plugin(FrameTimeDiagnosticsPlugin::default());
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
    app.add_plugin(OptionsPlugin); // menu
    //app.add_plugin(WaterMarkPlugin); // water mark //testing
    //app.add_plugin(CreatePlayerPlugin); // 
    //app.add_plugin(WorldBasicPlugin); // simple scene test
    app.add_plugin(NetworkMenuPlugin); // 
    app.add_plugin(CraftEventPlugin); // event testing...

    //app.add_plugin(PlayerPlugin);//conflict camera?
    
    //app.add_plugin(CraftPlayerPlugin); // event testing...
    app.add_plugin(CraftPhysics3DPlugin); // event testing...

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
    app.add_startup_system(set_window_icon);
    //app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    //app.add_state::<AppState>(); // state app
    
    //app.add_plugin(CraftBaseDataPlugin); // loading player data base
    
    // https://bevy-cheatbook.github.io/features/camera.html
    // for ui set up for camera need for render
    //app.add_startup_system(spawn_camera2d); // need this for bevy ui to render
    app.add_startup_system(spawn_camera3d); // 
    //note it need one camera at the time else log error on multiple camera active.

    //app.add_plugin(WaterMarkPlugin); // water mark //testing
    //app.add_plugin(SplashScreenPlugin); // Splash Screen //nope need rework

    
    //app.add_plugin(LoadingAssetUIPlugin); // ui loading
    app.add_plugin(LoadingAssetPlugin); // loading call
    app.add_plugin(MainMenuPlugin); // main menu
    app.add_plugin(OptionsPlugin); // menu
    app.add_plugin(CreatePlayerPlugin); // 

    app.add_plugin(CraftRayCastPlugin); // 
    //app.add_plugin(WorldBasicPlugin); // 
    app.add_plugin(NetworkMenuPlugin); // 
    app.add_plugin(CraftEventPlugin); // Events

    //app.add_plugin(PlayerPlugin);//conflict camera?
    //app.add_plugin(CraftPlayerPlugin); //

    app.add_plugin(HUDHotBarPlugin); // set up hot bar
    app.add_plugin(CraftPhysics3DPlugin); // set up physics

    // Test scene place 
    app.add_plugin(BaseWorldPlugin); //preload entity tests
    app.add_plugin(NetworkCraftPlugin); //
    //app.add_plugin(CraftSubAppPlugin); // testing app and sub app

    //check for state
    //app.add_startup_system(check_states); //
  }
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