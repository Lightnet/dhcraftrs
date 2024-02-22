/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

use crate::core::ui::menu::network::components::*;
use crate::core::ui::menu::network::styles::*;

pub fn spawn_network_menu(
  mut commands: Commands,
  asset_server:Res<AssetServer>
){
  println!("init network menu");
  let _network_menu_entity = build_network_menu(&mut commands, &asset_server);
}

pub fn despawn_network_menu(
  mut commands: Commands,
  network_menu_query:Query<Entity, With<NetworkMenu>>,
){
  if let Ok(network_menu_entity) = network_menu_query.get_single(){
    println!("remove network menu");
    commands.entity(network_menu_entity).despawn_recursive();
  }
}

pub fn build_network_menu(
  commands: &mut Commands,
  asset_server:&Res<AssetServer>
)-> Entity{
  let menu_entity = commands.spawn(
    (NodeBundle{
      style:get_network_menu_style(),
      //background_color: Color::RED.into(),
      ..default()
    },
    NetworkMenu,
    )
  ).with_children(|parent | {

    //text
      parent.spawn((
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Address", 
                get_button_text_style(&asset_server),
              )
            ],
            //alignment: TextAlignment::Center,
            ..default()
          },
          ..default()
        },
      ));
    // Server
    parent.spawn((
      TextBundle{
        text: Text{
          sections: vec![
            TextSection::new(
              "127.0.0.1", 
              get_button_text_style(&asset_server),
            )
          ],
          //alignment: TextAlignment::Center,
          ..default()
        },
        ..default()
      },
      ServerText
    ));

    // Text
    parent.spawn((
      TextBundle{
        text: Text{
          sections: vec![
            TextSection::new(
              "Address", 
              get_button_text_style(&asset_server),
            )
          ],
          //alignment: TextAlignment::Center,
          ..default()
        },
        ..default()
      },
    ));

    // PORT text
    parent.spawn((
      TextBundle{
        text: Text{
          sections: vec![
            TextSection::new(
              "5000", 
              get_button_text_style(&asset_server),
            )
          ],
          //alignment: TextAlignment::Center,
          ..default()
        },
        ..default()
      },
      PortText
    ));




    // HOST BUTTON
    parent.spawn((
      ButtonBundle {
        style:get_button_style(),
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
            //alignment: TextAlignment::Center,
            ..default()
          },
          ..default()
        }
      );
    });

    // JOIN BUTTON
    parent.spawn((
      ButtonBundle {
        style:get_button_style(),
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
            //alignment: TextAlignment::Center,
            ..default()
          },
          ..default()
        }
      );
    });

    // BACK BUTTON
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

  menu_entity
}