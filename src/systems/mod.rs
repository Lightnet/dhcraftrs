/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::core::api::CameraState;

pub fn spawn_camera2d(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
  let window = window_query.get_single().unwrap();

  commands.spawn(Camera2dBundle {
    transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ..default()
  });
}

pub fn spawn_camera3d(
  mut commands: Commands,
  //window_query: Query<&Window, With<PrimaryWindow>>
) {
  //let window = window_query.get_single().unwrap();
  commands.spawn(Camera3dBundle  {
      //transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
      //transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      transform: Transform::from_xyz(-4.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
  });
}

pub fn check_camera_state(
  camera_state: Res<State<CameraState>>
) {
  //println!("LOADED ASSETS...");
  match camera_state.0 {
    CameraState::Player => {
      println!("CAMERA: Player");
    }
    CameraState::Menu => {
      println!("CAMERA: Menu");
    }
    CameraState::Vehicle => {
      println!("CAMERA: Vehicle");
    }
    CameraState::Specter => {
      println!("CAMERA: Specter");
    }
    CameraState::Fixed => {
      println!("CAMERA: Fixed");
    }
  }
}

// Testing for add 3D model into scene
pub fn load_gltf_test01(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(SceneBundle {
    scene: asset_server.load("models/blockframe01.gltf#Scene0"),
    ..default()
  });
}
