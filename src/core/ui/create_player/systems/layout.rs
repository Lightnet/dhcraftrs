/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

use crate::{menu::{
  //components::*,
  styles::*,
}, core::ui::create_player::components::{CREATEPLAYERNAME, CREATEPLAYERNAMEBUTTON, PlayerNameText}};

pub fn spawn_create_player_menu(
  mut commands: Commands,
  asset_server:Res<AssetServer>
){
  let _create_player_menu_entity = build_create_player_menu(&mut commands, &asset_server);
}

pub fn despawn_create_player_menu(
  mut commands: Commands,
  main_menu_query:Query<Entity, With<CREATEPLAYERNAME>>,
){
  if let Ok(main_menu_entity) = main_menu_query.get_single(){
    commands.entity(main_menu_entity).despawn_recursive();
  }
}

pub fn build_create_player_menu(
  commands: &mut Commands,
  asset_server:&Res<AssetServer>
)-> Entity{
  let menu_loading_asset_entity = commands.spawn(
    (NodeBundle{
      style:MAIN_MENU_STYLE,
      //background_color: Color::RED.into(),
      ..default()
    },
    CREATEPLAYERNAME {},
    )
  ).with_children(|parent |{
    //title
    parent.spawn(
      NodeBundle{
        style:TITLE_STYLE,
        ..default()
      }
    ).with_children(|parent | {
      //image 1
      parent.spawn(ImageBundle{
        style:IMAGE_STYLE,
        image:asset_server.load("images/whiteblockblackline.png").into(),
        ..default()
      });

      //text
      parent.spawn((
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "player name", 
                get_title_text_style(&asset_server),
              )
            ],
            alignment: TextAlignment::Center,
            ..default()
          },
          ..default()
        },
        PlayerNameText
      ));
    });

    //new
    parent.spawn((
      ButtonBundle {
        style:BUTTON_STYLE,
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
      },
      CREATEPLAYERNAMEBUTTON {}
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Create", 
                get_button_text_style(&asset_server),
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