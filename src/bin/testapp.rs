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
  DefaultCraftPlugin, 
  Test01CraftPlugin
};

fn main() {//Entry point

  println!("Test App!");

  App::new()
    .add_plugins(DefaultPlugins)//window scree set up
    //.add_plugin(DefaultCraftPlugin) //craft set up
    .add_plugin(Test01CraftPlugin) //craft test
    .run();

}