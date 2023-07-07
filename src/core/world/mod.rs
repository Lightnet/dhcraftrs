/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
// for prefab or premade item or props to test scene
// stream world ???
pub mod prefab;

use bevy::prelude::*;

use super::{
  api::AppState, 
  physics::{
    create_ground
  }, 
  entity::{
    creature::player::systems::{create_entity_prototype_player, move_player_physics01, read_result_system_player}, 
    mesh::cube::{ 
      //create_entity_cube_pick, 
      create_entity_cube_physics
    }
  }
};

pub struct BaseWorldPlugin;
//testing need to bare minimal
// for console, headless server?
impl Plugin for BaseWorldPlugin{
  fn build(&self, app: &mut App){


    app.add_system(create_ground.in_schedule(OnEnter(AppState::Game)));

    //app.add_system(create_entity_cube.in_schedule(OnEnter(AppState::InGame)));
    //app.add_system(create_entity_cube_pick.in_schedule(OnEnter(AppState::InGame)));
    app.add_system(create_entity_cube_physics.in_schedule(OnEnter(AppState::Game)));

    app.add_system(create_entity_prototype_player.in_schedule(OnEnter(AppState::Game)));
    app.add_system(move_player_physics01.in_set(OnUpdate(AppState::Game)));
    app.add_system(read_result_system_player.in_set(OnUpdate(AppState::Game)));


  }

}
