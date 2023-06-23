/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::api::CameraState;
use crate::assets::MyAssets;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
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
      transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
  });
}

pub fn use_my_assets(_my_assets: Res<MyAssets>) {
  // do something using the asset handles from the resource
  println!("LOADED ASSETS...")
}

pub fn check_states(camera_state: Res<State<CameraState>>) {
  println!("LOADED ASSETS...");
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
  }
}

pub fn load_gltf_test01(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(SceneBundle {
    scene: asset_server.load("models/blockframe01.gltf#Scene0"),
    ..default()
  });
}
