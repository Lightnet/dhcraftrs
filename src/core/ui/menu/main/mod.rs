/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */


// Main Menu set up...

pub mod components;
pub mod styles;
pub mod systems;

use bevy::prelude::*;

use crate::core::{
  api::AppState, 
  ui::menu::main::systems::{
    layout::{
      spawn_main_menu, 
      despawn_main_menu}, 
    interactions::*
  }
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App){
    //println!("init main menu! plug in!");

    app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)));
    app.add_systems(
      (
        interact_with_play_button,
        interact_with_quit_button,
        interact_with_new_button,
        interact_with_online_button,
        interact_with_options_button
      ).in_set(OnUpdate(AppState::MainMenu))
    );

    app.add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
  }
}

//pub fn main_menu(){
  //println!("You are on Main Menu.")
//}