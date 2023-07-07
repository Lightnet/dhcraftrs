/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
pub mod systems;
pub mod components;
use bevy::prelude::*;

use crate::core::api::AppState;

use self::systems::{layout::{spawn_options_menu, despawn_options_menu}, interactions::interact_button_back};

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
  fn build(&self, app: &mut App){

    //create ui
    app.add_system(spawn_options_menu.in_schedule(OnEnter(AppState::OPTIONS)));
    //remove ui
    app.add_system(despawn_options_menu.in_schedule(OnExit(AppState::OPTIONS)));
    //input text
    //button event interact
    app.add_system(interact_button_back.in_set(OnUpdate(AppState::OPTIONS)));
  }
}