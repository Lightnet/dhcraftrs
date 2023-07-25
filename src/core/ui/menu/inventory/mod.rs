/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

pub mod systems;
pub mod components;
pub mod styles;

use bevy::prelude::*;

use self::systems::layout::spawn_inventory_menu;

pub struct InventoryMenuPlugin;

impl Plugin for InventoryMenuPlugin {
  fn build(&self, app: &mut App) {
    //Inventory menu

    app.add_systems(Startup, spawn_inventory_menu );

    //app.add_systems( OnEnter(GameState::MainMenu),spawn_main_menu);
    //app.add_systems( Startup, spawn_inventory_menu);
    //app.add_systems(Update, (
      //interact_play_button,
      //interact_online_button,
      //interact_settings_button,
      //interact_quit_button
    //).run_if(in_state(GameState::MainMenu)) );
    //app.add_systems( OnExit(GameState::MainMenu),despawn_main_menu);
  }
}