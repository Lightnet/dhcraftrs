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
#[allow(unused_imports)]
use super::{
  api::AppState,
  entity::{
    creature::player::systems::{read_result_system_player, create_entity_first_person_player, move_first_person_player_physics, move_first_person_player_cam}, 
    mesh::cube::{ 
      //create_entity_cube_pick, 
      create_entity_cube_physics, setup_ph_cube, place_holder_update, create_raycast_cube_physics
    }
  }, physics::create_ground
};

pub struct BaseWorldPlugin;
//testing need to bare minimal
// for console, headless server?
impl Plugin for BaseWorldPlugin{
  fn build(&self, app: &mut App){


    app.add_systems(OnEnter(AppState::Game),create_ground);

    //app.add_system(create_entity_cube.in_schedule(OnEnter(AppState::InGame)));
    //app.add_system(create_entity_cube_pick.in_schedule(OnEnter(AppState::InGame)));

    //app.add_systems(OnEnter(AppState::Game) ,create_entity_cube_physics);
    app.add_systems(OnEnter(AppState::Game) ,create_raycast_cube_physics);

    app.add_systems(OnEnter(AppState::Game), setup_ph_cube);
    //app.add_system(create_entity_prototype_player.in_schedule(OnEnter(AppState::Game)));
    app.add_systems(OnEnter(AppState::Game), create_entity_first_person_player);

    app.add_systems(Update,place_holder_update.run_if(in_state(AppState::Game)));
    //app.add_system(move_player_physics01.in_set(OnUpdate(AppState::Game)));
    app.add_systems(Update,move_first_person_player_physics);
    app.add_systems(Update,move_first_person_player_cam);
    app.add_systems(Update,read_result_system_player.run_if(in_state(AppState::Game)));


  }

}
