/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

// chest UI

use bevy::prelude::*;

pub struct ChestStoragePlugin;
 
impl Plugin for ChestStoragePlugin {
  fn build(&self, app: &mut App){
    //println!("init main menu! plug in!");

    //app.add_system(spawn_network_menu.in_schedule(OnEnter(AppState::NETWORK)));
    //app.add_systems(
      //(
      //interact_with_host_button,
      //interact_with_join_button
      //).in_set(OnUpdate(AppState::NETWORK))
    //);

    //app.add_system(despawn_network_menu.in_schedule(OnExit(AppState::NETWORK)));
  }
}