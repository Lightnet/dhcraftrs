/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

use bevy::{
  prelude::*, 
  //winit::WinitSettings,
  //tasks::IoTaskPool,
  //utils::Duration,
  window::{PresentMode, RequestRedraw, WindowResolution, WindowPlugin},
};
use dhcraftrs::{plugins::{
  //DefaultCraftPlugin, 
  //Test01CraftPlugin, 
  Test02CraftPlugin, 
  Test03CraftPlugin,
}, core::ui::{network_test::NetworkMenuPlugin, loading_asset::LoadingAssetsPlugin}};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {//Entry point

  println!("Test App!");

  App::new()
    //.add_plugins(DefaultPlugins)//window scree set up
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        //width: WIDTH,
        //height: HEIGHT,
        resolution: WindowResolution::new(WIDTH, HEIGHT).with_scale_factor_override(1.0),
        title: "Bevy Game Test".to_string(),
        resizable: false,
        ..default()
      }),
      ..default()
    }))
    //.add_plugin(DefaultCraftPlugin) //craft set up
    //.add_plugin(Test01CraftPlugin) //craft test
    //.add_plugin(Test02CraftPlugin) //craft test
    //.add_plugin(Test03CraftPlugin) //craft test
    //.add_plugin(NetworkMenuPlugin) //craft test

    .add_plugin(LoadingAssetsPlugin) //craft test
    .run();

}