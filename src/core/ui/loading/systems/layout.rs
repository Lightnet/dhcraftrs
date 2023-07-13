/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

use crate::core::ui::{loading::components::LoadingAsset, menu::main::styles::*};

pub fn spawn_loading_asset_ui(
  mut commands: Commands,
  asset_server:Res<AssetServer>
){
  let _loading_asset_entity = build_loading_asset_ui(&mut commands, &asset_server);
}

pub fn despawn_loading_asset_ui(
  mut commands: Commands,
  loading_asset_query:Query<Entity, With<LoadingAsset>>,
){
  //println!("CHECK ENTITY FOR DELETE LOADING ASSETS...");
  if let Ok(loading_asset_entity) = loading_asset_query.get_single(){
    //println!("FOUND LOADING ASSETS...");
    commands.entity(loading_asset_entity).despawn_recursive();
  }
}

pub fn build_loading_asset_ui(
  commands: &mut Commands,
  asset_server:&Res<AssetServer>
)-> Entity{
  let menu_loading_asset_entity = commands.spawn(
    (NodeBundle{
      style:MAIN_MENU_STYLE,
      //background_color: Color::RED.into(),
      ..default()
    },
    LoadingAsset,
    )
  ).with_children(|parent |{
    //title
    parent.spawn(
      NodeBundle{
        style:get_title_style(),
        ..default()
      }
    ).with_children(|parent | {
      //image 1
      parent.spawn(ImageBundle{
        style:get_image_style(),
        image:asset_server.load("images/whiteblockblackline.png").into(),
        ..default()
      });

      //text
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Loading...", 
                get_title_text_style(&asset_server),
              )
            ],
            alignment: TextAlignment::Center,
            ..default()
          },
          ..default()
        }
      );
    });
  })
   .id();

   menu_loading_asset_entity
}