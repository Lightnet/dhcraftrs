/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

// build_cube(&mut commands, &assets_server);

pub fn set_up_test(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
){
  build_cube(&mut commands, &asset_server, &mut meshes, &mut materials);

  // light
  commands.spawn(PointLightBundle {
    point_light: PointLight {
        intensity: 1500.0,
        shadows_enabled: true,
        ..default()
    },
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..default()
  });
  // camera
  //commands.spawn(Camera3dBundle {
      //transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      //..default()
  //});
}

pub fn build_cube(
  commands: &mut Commands,
  _asset_server: &Res<AssetServer>,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<StandardMaterial>>
){
  // cube
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
  });
}