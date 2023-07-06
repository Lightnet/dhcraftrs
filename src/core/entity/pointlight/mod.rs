/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
use bevy::prelude::*;

#[allow(dead_code)]
pub fn setup_pointlight(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  create_pointlight(&mut commands, &mut meshes, &mut materials);
}


#[allow(dead_code, unused_variables)]
pub fn create_pointlight(
  commands: &mut Commands,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<StandardMaterial>>,
){
 commands.spawn(PointLightBundle {
   point_light: PointLight {
       intensity: 1500.0,
       shadows_enabled: true,
       ..default()
   },
   transform: Transform::from_xyz(4.0, 8.0, 4.0),
   ..default()
 }).insert(Name::new("light"));
}