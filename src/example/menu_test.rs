
// Testing the simple scene

//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;

use dhcraftrs::core::{systems::spawn_camera3d, ui::menu::inventory::InventoryMenuPlugin};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Update, spawn_camera3d)
    .add_plugins(InventoryMenuPlugin)
    .run();
}