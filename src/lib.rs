
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
pub mod assets;
//use menu::*;


// Test public function call
pub fn test_print(){
  println!("TEST ME PUB!!")
}

// API? for struct stuff public
#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
pub enum AppState{
  //#[default]
  MainMenu,
  Next,
  InGame,
  EndGame,
  LoadingGame,
  #[default]//note that if loading error when not first started. when fn use_my_assets error. 
  AssetLoading,
  BootingApp,
  StartScreen,
  ErrorScreen,
}







