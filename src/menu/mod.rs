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

use self::systems::layout::{spawn_main_menu, despawn_main_menu};
use crate::{
  api::AppState, 
  menu::systems::interactions::{
    interact_with_play_button,
    interact_with_quit_button, 
    interact_with_new_button
  }
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App){
    //app.add_startup_system(main_menu);
    println!("init main menu! plug in!");

    app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)));
    app.add_systems(
      (
        interact_with_play_button,
        interact_with_quit_button,
        interact_with_new_button
      ).in_set(OnUpdate(AppState::MainMenu))
    );
    //app.add_system(
      //interact_with_quit_button.in_set(OnEnter)
    //);

    app.add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
  }
}

//pub fn main_menu(){
  //println!("You are on Main Menu.")
//}