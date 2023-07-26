/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

 use bevy::prelude::*;
 //use bevy_mod_picking::prelude::*;

pub fn create_pick_cube(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  commands.spawn((
    PbrBundle{
      mesh: meshes.add(Mesh::from(shape::Capsule {  
        radius:0.9,
        ..default()
      })),
      material: materials.add(StandardMaterial {
        base_color: Color::rgba(0.5, 0.5, 0.5, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..default()
      }),
      //transform: Transform::from_xyz(0.0, 0.0, 0.0),
      ..default()
    },           // The `bevy_picking_raycast` backend works with meshes
    //PickableBundle::default(),      // Makes the entity pickable
    //RaycastPickTarget::default()    // Marker for the `bevy_picking_raycast` backend
  ));
}