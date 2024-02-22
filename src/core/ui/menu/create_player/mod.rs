/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

// https://bevy-cheatbook.github.io/input/char.html

pub mod systems;
pub mod components;
use bevy::prelude::*;

use crate::core::api::AppState;

use self::systems::{
  layout::{
    spawn_create_player_menu, 
    despawn_create_player_menu
  }, 
  interactions::{ interact_button_create_player, interact_button_back}
};

pub struct CreatePlayerPlugin;

impl Plugin for CreatePlayerPlugin {
  fn build(&self, app: &mut App){
    //create ui
    app.add_systems(OnEnter(AppState::CREATEPLAYERNAME), spawn_create_player_menu);
    //remove ui
    app.add_systems(OnExit(AppState::CREATEPLAYERNAME), despawn_create_player_menu);
    
    //button event interact //input text
    app.add_systems(Update, 
      (
        //player_name_text_update,
        interact_button_create_player, 
        interact_button_back
      ).run_if(in_state(AppState::CREATEPLAYERNAME))
    );
  }
}
