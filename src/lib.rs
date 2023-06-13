
//===============================================
// LIB and not bin
//===============================================
use bevy::{
  prelude::*, 
  winit::WinitSettings,
  //tasks::IoTaskPool,
  //utils::Duration
};

pub mod menu;
pub mod systems;
pub mod assets;
pub mod api;
pub mod plugins;
//use menu::*;

// Test public function call
pub fn test_print(){
  println!("TEST ME PUB!!")
}

// API? for struct stuff public







