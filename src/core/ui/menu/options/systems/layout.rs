/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

use crate::{
  core::ui::menu::{
    main::styles::*, 
    options::components::{
      MenuOptions, 
      BackButton
    }
  }
};

pub fn spawn_options_menu(
  mut commands: Commands,
  asset_server:Res<AssetServer>
){
  let _create_options_menu_entity = build_create_options_menu(&mut commands, &asset_server);
}

pub fn despawn_options_menu(
  mut commands: Commands,
  main_menu_query:Query<Entity, With<MenuOptions>>,
){
  if let Ok(main_menu_entity) = main_menu_query.get_single(){
    commands.entity(main_menu_entity).despawn_recursive();
  }
}

pub fn build_create_options_menu(
  commands: &mut Commands,
  asset_server:&Res<AssetServer>
)-> Entity{
  let menu_loading_asset_entity = commands.spawn(
    (NodeBundle{
      style:get_main_menu_style(),
      //background_color: Color::RED.into(),
      ..default()
    },
    MenuOptions,
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
    });
    
    parent.spawn((
      ButtonBundle {
        style:get_button_style(),
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
      },
      BackButton
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Back", 
                get_button_text_style(&asset_server),
              )
            ],
            //alignment: TextAlignment::Center,
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