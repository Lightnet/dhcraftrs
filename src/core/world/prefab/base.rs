/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

//test load scene
// app.add_system(basic_scene_test.in_schedule(OnEnter(AppState::InGame)));
pub fn basic_scene_test(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  // plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });
  // cube
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
  });
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

pub fn basic_scene_player_test(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  // plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });

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

}