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

use self::systems::{layout::{spawn_create_player_menu, despawn_create_player_menu}, interactions::{player_name_text_update, interact_button_create_player, interact_button_back}};

pub struct CreatePlayerPlugin;

impl Plugin for CreatePlayerPlugin {
  fn build(&self, app: &mut App){
    //app.add_state::<AppState>();//state app
    //app.add_startup_system(set_loadingasset_menu);
    //app.add_system(create_player_text_input);
    //app.add_system(player_name_text_update.in_schedule(OnUpdate(AppState::CREATEPLAYERNAME))); //nope ???

    //create ui
    app.add_system(spawn_create_player_menu.in_schedule(OnEnter(AppState::CREATEPLAYERNAME)));
    //remove ui
    app.add_system(despawn_create_player_menu.in_schedule(OnExit(AppState::CREATEPLAYERNAME)));
    //input text
    app.add_system(player_name_text_update.in_set(OnUpdate(AppState::CREATEPLAYERNAME)));
    //button event interact
    //app.add_system(interact_button_create_player.in_set(OnUpdate(AppState::CREATEPLAYERNAME)));
    //app.add_system(interact_button_back.in_set(OnUpdate(AppState::CREATEPLAYERNAME)));
  }
}
