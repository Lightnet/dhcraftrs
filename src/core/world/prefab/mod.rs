/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;
#[allow(unused_imports)]
use crate::core::api::AppState;

pub struct WorldBasicPlugin;

impl Plugin for WorldBasicPlugin{
  fn build(&self, app: &mut App){
    //TODOLIST
    //test loading entity
    //app.add_systems(Startup, basic_scene_test.in_schedule(OnEnter(AppState::Game)));
    app.add_systems(Startup, basic_scene_test);
  }
}

//test load scene
// app.add_system(basic_scene_test.in_schedule(OnEnter(AppState::InGame)));
pub fn basic_scene_test(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  // plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
    ..default()
  });
  // cube
  commands.spawn(PbrBundle {
    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
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
    mesh: meshes.add(Plane3d::default().mesh().size(5., 5.)),
    material: materials.add(Color::WHITE),
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