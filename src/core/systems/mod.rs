/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
/*
  Information:
    For basic set up for UI stuff and other common for testing builds.

*/

//use bevy::app::AppExit;
#[allow(unused_imports)]
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
//use bevy_mod_raycast::RaycastSource;
//use bevy_mod_picking::prelude::RaycastPickCamera;
#[allow(unused_imports)]
use crate::core::api::{CameraState, AppState};

//use super::raycast::MyRaycastSet;

pub fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
  TextStyle {
    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    font_size:28.0,
    color: Color::WHITE,
  }
}

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
  commands.spawn(
    (
      Camera3dBundle  {
        //transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        //transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        //transform: Transform::from_xyz(-4.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        transform: Transform::from_xyz(0., 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
      },
      //RaycastSource::<MyRaycastSet>::new() // Designate the camera as our source
    //RaycastPickCamera::default(),
    )
  );
}

pub fn system_query_info(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  //mut player_info: ResMut<PlayerInfo>,
){
  //if keyboard_input.pressed(KeyCode::A) {
    //println!("player_info: {:?}", player_info);
    //println!("player Name: {:}", player_info.name);
  //}

  //keyboard_input.get_just_released()

  if keyboard_input.just_released(KeyCode::KeyA) {
    //println!("player_info: {:?}", player_info);
    //println!("player Name: {:}", player_info.name);
  }

  if keyboard_input.just_pressed(KeyCode::KeyD) {
    //println!("[[player_info]]: {:?}", player_info);
  }
}


/*
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
*/

// Testing for add 3D model into scene
pub fn load_gltf_test01(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(SceneBundle {
    scene: asset_server.load("models/blockframe01.gltf#Scene0"),
    ..default()
  });
}

#[allow(dead_code)]
pub fn set_game_state(
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  app_state_next_state.set(AppState::Game);
}
#[allow(dead_code)]
pub fn check_app_states(
  app_state_next_state:ResMut<NextState<AppState>>,
){
  println!("app_state_next_state: {:?}", app_state_next_state.0);
}
#[allow(dead_code)]
pub fn set_network_menu(
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  app_state_next_state.set(AppState::NETWORK);
}