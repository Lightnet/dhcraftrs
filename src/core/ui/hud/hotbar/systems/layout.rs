/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

use crate::{
  core::{ui::hud::hotbar::{
    styles::{
      HOT_BAR_STYLE, 
      ICON_HOT_BAR_STYLE, NORMAL_ICON_BUTTON_COLOR
    }, 
    components::*
  }, styles::get_button_text_style}
};

pub fn spawn_hud_hot_bars(
  mut commands: Commands,
  asset_server:Res<AssetServer>,
){
  let _main_menu_entity = build_hot_bar(&mut commands, &asset_server);
}

pub fn build_hot_bar(
  commands: &mut Commands,
  asset_server:&Res<AssetServer>
)-> Entity{
  let hot_bar_entity = commands.spawn(
    (NodeBundle{
      style:HOT_BAR_STYLE,
      //background_color: Color::RED.into(),
      ..default()
    },
    HOTBAR,
    )
  ).with_children(|parent |{

    //HOTBAR 1
    parent.spawn((
      ButtonBundle {
        style:ICON_HOT_BAR_STYLE,
        background_color: NORMAL_ICON_BUTTON_COLOR.into(),
        ..default()
      },
      HOTBAR0
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "No1", 
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

    //HOTBAR 1
    parent.spawn((
      ButtonBundle {
        style:ICON_HOT_BAR_STYLE,
        background_color: NORMAL_ICON_BUTTON_COLOR.into(),
        ..default()
      },
      HOTBAR1
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "No2", 
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


  hot_bar_entity//return
}

pub fn despawn_hud_hot_bars(
  mut commands: Commands,
  hot_bar_query:Query<Entity, With<HOTBAR>>,
){
  if let Ok(hot_bar_entity) = hot_bar_query.get_single(){
    commands.entity(hot_bar_entity).despawn_recursive();
  }
}