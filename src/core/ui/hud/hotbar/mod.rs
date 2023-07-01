/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//===============================================
//
//===============================================

pub mod systems;
pub mod components;
pub mod styles;

use bevy::prelude::*;

use crate::{core::{ui::hud::hotbar::systems::layout::{spawn_hud_hot_bars, despawn_hud_hot_bars}, api::AppState}};

pub struct HUDHotBarPlugin;

impl Plugin for HUDHotBarPlugin {
  fn build(&self, app: &mut App){
    //app.add_startup_system(main_menu);
    println!("init main menu! plug in!");

    app.add_system(spawn_hud_hot_bars.in_schedule(OnEnter(AppState::InGame)));
    //app.add_systems(
      //(
        //interact_with_play_button,
        //interact_with_quit_button,
        //interact_with_new_button,
        //interact_with_online_button,
        //interact_with_options_button
      //).in_set(OnUpdate(AppState::MainMenu))
    //);
    //app.add_system(interact_with_quit_button.in_set(OnEnter));

    app.add_system(despawn_hud_hot_bars.in_schedule(OnExit(AppState::InGame)));
  }
}