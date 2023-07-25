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
    app.add_systems(OnEnter(AppState::OPTIONS),spawn_options_menu);
    //remove ui
    app.add_systems(OnExit(AppState::OPTIONS),despawn_options_menu);
    //input text
    //button event interact
    app.add_systems(Update,interact_button_back.run_if(in_state(AppState::OPTIONS)));
  }
}