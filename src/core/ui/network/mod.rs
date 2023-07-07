/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

mod components;
mod styles;
mod systems;

use bevy::prelude::*;
 
use crate::{
  core::{ui::network::systems::{
    layout::{
      spawn_network_menu, 
      despawn_network_menu
    }, interactions::{
      interact_with_host_button, 
      interact_with_join_button, 
      interact_with_back_button
    }
  }, api::AppState}
};
 
pub struct NetworkMenuPlugin;
 
impl Plugin for NetworkMenuPlugin {
  fn build(&self, app: &mut App){
    println!("init network menu! plug in!");

    app.add_system(spawn_network_menu.in_schedule(OnEnter(AppState::NETWORK)));
    app.add_systems(
      (
      interact_with_host_button,
      interact_with_join_button,
      interact_with_back_button
      ).in_set(OnUpdate(AppState::NETWORK))
    );

    app.add_system(despawn_network_menu.in_schedule(OnExit(AppState::NETWORK)));
  }
}

pub fn set_network_menu(
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  app_state_next_state.set(AppState::NETWORK);
}