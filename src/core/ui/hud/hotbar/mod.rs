/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

pub mod systems;
pub mod components;
pub mod styles;

use bevy::prelude::*;

#[allow(unused_imports)]
use crate::core::{
    ui::hud::hotbar::systems::{
      layout::{
        spawn_hud_hot_bars, 
        despawn_hud_hot_bars
      }, 
      interactions::interact_hot_bar_id_button
    }, 
    api::AppState
  };

pub struct HUDHotBarPlugin;

impl Plugin for HUDHotBarPlugin {
  fn build(&self, app: &mut App){
    // setup enity hot bar UI
    app.add_systems(OnEnter(AppState::Game),spawn_hud_hot_bars);
    // update listen to button events
    app.add_systems(Update,
      interact_hot_bar_id_button
      .run_if(in_state(AppState::Game))
    );
    // remove entity hot bar UI
    app.add_systems(OnExit(AppState::Game),despawn_hud_hot_bars);
  }
}