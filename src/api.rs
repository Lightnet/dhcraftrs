/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//===============================================
// for struct and variables
//===============================================

use bevy::prelude::*;

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

#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
pub enum CameraState{
  //#[default]
  Player,
  #[default]
  Menu,
  Vehicle,
  Specter,
}