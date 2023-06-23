/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
#[warn(unused_imports)]

use bevy::prelude::*;

use crate::core::ui::network_test::{
  components::*, 
  styles::*,
};

pub fn spawn_network_menu(
  mut commands: Commands,
  asset_server:Res<AssetServer>
){
  let main_menu_entity = build_network_menu(&mut commands, &asset_server);
}

pub fn despawn_network_menu(
  mut commands: Commands,
  network_menu_query:Query<Entity, With<NetworkMenu>>,
){
  if let Ok(network_menu_entity) = network_menu_query.get_single(){
    commands.entity(network_menu_entity).despawn_recursive();
  }
}

pub fn build_network_menu(
  commands: &mut Commands,
  asset_server:&Res<AssetServer>
)-> Entity{
  let menu_entity = commands.spawn(
    (NodeBundle{
      style:NETWORK_MENU_STYLE,
      //background_color: Color::RED.into(),
      ..default()
    },
    NetworkMenu {},
    )
  ).with_children(|parent | {
    //play
    parent.spawn((
      ButtonBundle {
        style:BUTTON_STYLE,
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
      },
      HostNetworkButton {}
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Host", 
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

    //play
    parent.spawn((
      ButtonBundle {
        style:BUTTON_STYLE,
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
      },
      JoinNetworkButton {}
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Join", 
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

  menu_entity
}