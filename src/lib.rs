
//===============================================
//
//===============================================
use bevy::{
  prelude::*, 
  winit::WinitSettings,
  //tasks::IoTaskPool,
  //utils::Duration
};

pub mod menu;
pub mod systems;
//use menu::*;


// Test public function call
pub fn test_print(){
  println!("TEST ME PUB!!")
}

// API? for struct stuff public
#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
pub enum AppState{
  #[default]
  MainMenu,
  InGame,
}







