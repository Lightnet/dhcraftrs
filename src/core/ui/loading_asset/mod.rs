/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
pub mod components;
pub mod systems;
use bevy::{
  prelude::*,
  //window::PrimaryWindow
};

use crate::{
  api::AppState,
  core::ui::loading_asset::systems::layout::{
    despawn_loading_asset_menu,
    spawn_loading_asset_menu
  }
};

pub struct LoadingAssetUIPlugin;

impl Plugin for LoadingAssetUIPlugin {
  fn build(&self, app: &mut App){
    //need to fixed or once time loading for assets?

    //app.add_state::<AppState>();//state app
    //set up once
    app.add_startup_system(spawn_loading_asset_menu);
    //strange bug loop and add twice
    //app.add_system(spawn_loading_asset_menu.in_schedule(OnEnter(AppState::AssetLoading)));
    //app.add_system(spawn_loading_asset_menu.in_set(OnEnter(AppState::AssetLoading)));//nope not correct code
    app.add_system(despawn_loading_asset_menu.in_schedule(OnExit(AppState::AssetLoading)));
  }
}

#[allow(dead_code)]
fn set_loadingasset_menu(
  //mut commands: Commands,
  mut app_state_next_state:ResMut<NextState<AppState>>,
  //window_query: Query<&Window, With<PrimaryWindow>>,
){
  //let window = window_query.get_single().unwrap();
  app_state_next_state.set(AppState::AssetLoading);

  //testing
  //commands.spawn(Camera2dBundle {
    //transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    //..default()
  //});
}