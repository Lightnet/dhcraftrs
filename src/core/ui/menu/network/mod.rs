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

use crate::core::api::AppState;
use crate::core::ui::menu::network::systems::layout::*;
use crate::core::ui::menu::network::systems::interactions::*;
pub struct NetworkMenuPlugin;
 
impl Plugin for NetworkMenuPlugin {
  fn build(&self, app: &mut App){
    //println!("init network menu! plug in!");

    app.add_systems(OnEnter(AppState::NETWORK),spawn_network_menu);
    app.add_systems(
      Update,
      (
      interact_with_host_button,
      interact_with_join_button,
      interact_with_back_button
      ).run_if(in_state(AppState::NETWORK))
    );

    app.add_systems(OnExit(AppState::NETWORK),despawn_network_menu);
  }
}

pub fn set_network_menu(
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  app_state_next_state.set(AppState::NETWORK);
}