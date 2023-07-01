/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//===============================================
// Assets for interal use ?
//===============================================

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::{api::AppState, ui::loading_asset::components::LoadingAsset};

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
pub struct MyAssets {
  //#[asset(path = "images/player.png")]
  //player: Handle<Image>,
  //#[asset(path = "walking.ogg")]
  //walking: Handle<AudioSource>,

  #[asset(path = "images/whiteblockblackline.png")]
  blockwhiteblackline: Handle<Image>,
}

pub struct LoadingAssetPlugin;

impl Plugin for LoadingAssetPlugin{
  fn build(&self, app: &mut App){
    //println!("init loading assets! plug in!");
    //app.add_startup_system(spawn_camera2d);//need this for bevy ui to render
    //loading assets state
    app.add_loading_state(
      LoadingState::new(AppState::AssetLoading)
        .continue_to_state(AppState::MainMenu)
        //.on_failure_continue_to_state(next)
        //.continue_to_state(AppState::AssetLoading)//test
    );
    //loading assets
    app.add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading);
    //assets do something
    app.add_system(use_my_assets.in_schedule(OnEnter(AppState::MainMenu)));
    
  }
}

pub fn use_my_assets(
  mut _commands: Commands,
  _my_assets: Res<MyAssets>,
  _loading_asset_query:Query<Entity, With<LoadingAsset>>,
) {
  //if let Ok(loading_asset_entity) = loading_asset_query.get_single(){
    //commands.entity(loading_asset_entity).despawn_recursive();
  //}
  // do something using the asset handles from the resource
  //println!("LOADED ASSETS...");
}

//pub fn use_my_assets(_my_assets: Res<MyAssets>) {
  // do something using the asset handles from the resource
  //println!("LOADED ASSETS...")
//}

/*
app
.add_loading_state(
      LoadingState::new(AppState::AssetLoading)
          .continue_to_state(AppState::MainMenu)
    )
    .add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading)
    .add_system(use_my_assets.in_schedule(OnEnter(AppState::MainMenu)))

 */