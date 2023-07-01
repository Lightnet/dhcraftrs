/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//===============================================
// LIB and not bin
//===============================================
//use bevy::{
  //prelude::*, 
  //winit::WinitSettings,
  //tasks::IoTaskPool,
  //utils::Duration
//};

//public for library access to bin 
pub mod core;
pub mod menu;
pub mod systems;
pub mod plugins;

// Test public function call
pub fn test_print(){
  println!("TEST ME PUB!!")
}

// API? for struct stuff public







