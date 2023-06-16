/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */


use bevy::{
  prelude::*, 
  //winit::WinitSettings,
  //tasks::IoTaskPool,
  //utils::Duration
};
use dhcraftrs::plugins::{
  //DefaultCraftPlugin, 
  //Test01CraftPlugin, 
  Test02CraftPlugin,
};

fn main() {//Entry point

  println!("Test App!");

  App::new()
    .add_plugins(DefaultPlugins)//window scree set up
    //.add_plugin(DefaultCraftPlugin) //craft set up
    //.add_plugin(Test01CraftPlugin) //craft test
    .add_plugin(Test02CraftPlugin) //craft test
    .run();

}