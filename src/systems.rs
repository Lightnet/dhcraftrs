

//===============================================
// for editor and global config?
//===============================================

use bevy::app::AppExit;
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