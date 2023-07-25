/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::core::api::AppState;

use self::systems::layout::{spawn_loading_asset_ui, despawn_loading_asset_ui};

pub struct LoadingAssetUIPlugin;

impl Plugin for LoadingAssetUIPlugin {
  fn build(&self, app: &mut App){
    //need to fixed or once time loading for assets?
    //set up
    app.add_systems(Startup, spawn_loading_asset_ui);
    app.add_systems(OnExit(AppState::AssetLoading), despawn_loading_asset_ui);
  }
}
