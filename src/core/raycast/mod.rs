/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

pub mod systems;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

// https://github.com/aevyrie/bevy_mod_picking/blob/v0.13/examples/event_listener.rs

pub struct CraftRayCastPlugin;

impl Plugin for CraftRayCastPlugin{//main entry point still in testing...

  fn build(&self, app: &mut App){
    //app.add_plugins(DefaultPickingPlugins);
    app.add_plugins(
      DefaultPickingPlugins
          .build()
          .disable::<DefaultHighlightingPlugin>(),
    );
    app.add_startup_system(setup_test);

  }

}

pub fn setup_test(
  mut commands: Commands,
  //app: &mut App,
){
    commands.spawn((
      PbrBundle::default(),           // The `bevy_picking_raycast` backend works with meshes
      //RaycastPickCamera::default(),
      PickableBundle::default(),      // Makes the entity pickable
      RaycastPickTarget::default()    // Marker for the `bevy_picking_raycast` backend
    ));

    //commands.spawn((
      //Camera3dBundle::default(),
      //RaycastPickCamera::default(),   // Enable picking with this camera
    //));
}