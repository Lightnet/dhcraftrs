#![allow(unused_variables)]
/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
 /*
    Information:
      Need to center all stuff here?
      For better sorting.
    


  */

pub mod components;
pub mod systems;

use bevy::prelude::*;
//use crate::core::api::AppState;

pub struct CraftPlayerPlugin;

impl Plugin for CraftPlayerPlugin{

  fn build(&self, app: &mut App){

    //app.add_system(create_entity_prototype_player.in_schedule(OnEnter(AppState::InGame)));
    //app.add_system(player_movement02.in_set(OnUpdate(AppState::InGame)));

    //app.add_startup_system(set_app_state_game);
  }
}

pub struct CraftPlayerTestPlugin;

impl Plugin for CraftPlayerTestPlugin{

  fn build(&self, app: &mut App){

    //app.add_system(create_entity_player.in_schedule(OnEnter(AppState::InGame)));
    //app.add_system(create_entity_prototype_player.in_schedule(OnEnter(AppState::InGame)));
    //app.add_system(player_movement02.in_set(OnUpdate(AppState::InGame)));

    //app.add_startup_system(set_app_state_game);
  }
}
