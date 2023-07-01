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

use crate::{core::{ui::hud::hotbar::systems::{layout::{spawn_hud_hot_bars, despawn_hud_hot_bars}, interactions::{interact_hot_bar_0_button, interact_hot_bar_1_button}}, api::AppState}};

pub struct HUDHotBarPlugin;

impl Plugin for HUDHotBarPlugin {
  fn build(&self, app: &mut App){
    // setup enity hot bar UI
    app.add_system(spawn_hud_hot_bars.in_schedule(OnEnter(AppState::InGame)));
    // update listen to button events
    app.add_system(
        interact_hot_bar_0_button
      .in_set(OnUpdate(AppState::InGame))
    );
    // remove entity hot bar UI
    app.add_system(despawn_hud_hot_bars.in_schedule(OnExit(AppState::InGame)));
  }
}