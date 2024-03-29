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
  MainMenu, // use
  //Next,
  LoadingGame, // not yet
  Game, // use
  EndGame, // not yet
  //LoadingScene,
  //LoadingWorld,
  //SCENE,
  #[default]//note that if loading error when not first started. when fn use_my_assets error. 
  AssetLoading, // use
  //BootingApp,
  //StartScreen,
  //ErrorScreen,
  OPTIONS, // use
  CREATEPLAYERNAME, // use
  NETWORK, // use
  SERVER, // use
  CLIENT, // use
}

#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
pub enum GameState{
  #[default]
  Loading,
  //Start,
  Game,
  Unloading,
  //End,
}

#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
pub enum CameraState{
  //#[default]
  Player,
  #[default]
  Menu,
  Creature,
  Vehicle,
  Specter,
  Fixed,
  //CutScene,
}

#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
pub enum NetworkState{
  #[default]
  STANDALONE,
  //#[default]
  SERVER,
  CLIENT,
  PEERTOPEERSERVER,
  PEERTOPEERCLIENT,
}