/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
/*
  Information:
    Need to preload assets. For easy access as keep the loading reduce some degree.

    Assets for internal use?
 */

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::api::AppState;

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
pub struct MyAssets {
  //#[asset(path = "images/player.png")]
  //player: Handle<Image>,
  //#[asset(path = "walking.ogg")]
  //walking: Handle<AudioSource>,
  //#[asset(path = "models/blockframe01.gltf")]//nope
  #[asset(path = "models/blockframe01.gltf#Scene0")]
  blockframe01: Handle<Scene>,

  #[asset(path = "models/character/male01/male01.gltf#Scene0")]
  male01: Handle<Scene>,

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

// https://bevy-cheatbook.github.io/assets/assetserver.html
#[allow(dead_code,unused_mut,unused_variables)]
pub fn use_my_assets(
  mut commands: Commands,
  my_assets: Res<MyAssets>,
  //_loading_asset_query:Query<Entity, With<LoadingAsset>>,
) {
  println!("loaded?");
  //if let Ok(loading_asset_entity) = loading_asset_query.get_single(){
    //commands.entity(loading_asset_entity).despawn_recursive();
  //}
  // do something using the asset handles from the resource
  //println!("LOADED ASSETS...");

  commands.spawn(SceneBundle {
    scene: my_assets.male01.clone(),
    ..Default::default()
  });

  //commands.spawn(SceneBundle {
    //scene: my_assets.blockframe01.clone(),
    //..Default::default()
  //});
}
