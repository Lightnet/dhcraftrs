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

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
  //#[asset(path = "images/player.png")]
  //player: Handle<Image>,
  //#[asset(path = "walking.ogg")]
  //walking: Handle<AudioSource>,

  #[asset(path = "images/whiteblockblackline.png")]
  blockwhiteblackline: Handle<Image>,
  
}

//pub fn use_my_assets(_my_assets: Res<MyAssets>) {
  // do something using the asset handles from the resource
  //println!("LOADED ASSETS...")
//}


/*
.add_loading_state(
      LoadingState::new(AppState::AssetLoading)
          .continue_to_state(AppState::MainMenu)
    )
    .add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading)
    .add_system(use_my_assets.in_schedule(OnEnter(AppState::MainMenu)))

 */