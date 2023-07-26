/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

pub mod systems;

use bevy::prelude::*;
//use bevy_mod_picking::prelude::*;
use bevy_mod_raycast::{
  DefaultRaycastingPlugin, 
  RaycastSystem, 
  print_intersections, 
  RaycastSource, 
  RaycastMethod, 
  RaycastMesh,
  //RaycastPluginState
};
use bevy_mod_raycast::DefaultPluginState as RaycastPluginState;

// https://github.com/aevyrie/bevy_mod_picking/blob/v0.13/examples/event_listener.rs

#[derive(Clone, Reflect, Component)]
pub struct MyRaycastSet;

pub struct CraftRayCastPlugin;

impl Plugin for CraftRayCastPlugin{//main entry point still in testing...

  fn build(&self, app: &mut App){
    //app.add_plugins(DefaultPickingPlugins);
    //app.add_plugins(
      //DefaultPickingPlugins
        //.build()
        //.disable::<DefaultHighlightingPlugin>()
        //.disable::<DebugPickingPlugin>(),
    //);
    //app.add_startup_system(setup_test);
    app.add_plugins(DefaultRaycastingPlugin::<MyRaycastSet>::default());
    app.add_systems(
      First,
      update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MyRaycastSet>),
    );
    app.add_systems(Startup, (
      debug_raycast_setup, 
      print_intersections::<MyRaycastSet>,
      raycast_mesh_test_setup
    ));
    //app.add_systems(Startup, (setup, print_intersections::<MyRaycastSet>));
  }

}

// Update our `RaycastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
  mut cursor: EventReader<CursorMoved>,
  mut query: Query<&mut RaycastSource<MyRaycastSet>>,
) {
  // Grab the most recent cursor event if it exists:
  let Some(cursor_moved) = cursor.iter().last() else { return };
  for mut pick_source in &mut query {
      pick_source.cast_method = RaycastMethod::Screenspace(cursor_moved.position);
  }
}

fn debug_raycast_setup(
  mut commands: Commands,
  //mut meshes: ResMut<Assets<Mesh>>,
  //mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.insert_resource(RaycastPluginState::<MyRaycastSet>::default().with_debug_cursor());

  //commands
    //.spawn(Camera3dBundle {
      //transform: Transform::from_xyz(-2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
      //..Default::default()
    //})
    //.insert(RaycastSource::<MyRaycastSet>::new()); // Designate the camera as our source

  //commands
    //.spawn(PbrBundle {
      //mesh: meshes.add(Mesh::try_from(shape::Icosphere::default()).unwrap()),
      //material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
      //..Default::default()
    //})
    //.insert(RaycastMesh::<MyRaycastSet>::default()); // Make this mesh ray cast-able

}

fn raycast_mesh_test_setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {

  //commands
    //.spawn(Camera3dBundle {
      //transform: Transform::from_xyz(-2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
      //..Default::default()
    //})
    //.insert(RaycastSource::<MyRaycastSet>::new()); // Designate the camera as our source

  commands
    .spawn(PbrBundle {
      mesh: meshes.add(Mesh::try_from(shape::Icosphere::default()).unwrap()),
      material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
      ..Default::default()
    })
    .insert(RaycastMesh::<MyRaycastSet>::default()); // Make this mesh ray cast-able

}



/*
//mod_picking
#[allow(dead_code)]
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
*/