/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

use crate::core::ui::menu::main::{
  components::*, 
  styles::*
};

pub fn spawn_main_menu(
  mut commands: Commands,
  asset_server:Res<AssetServer>,
){
  let _main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
  mut commands: Commands,
  main_menu_query:Query<Entity, With<MainMenu>>,
){
  if let Ok(main_menu_entity) = main_menu_query.get_single(){
    commands.entity(main_menu_entity).despawn_recursive();
  }
}

pub fn build_main_menu(
  commands: &mut Commands,
  asset_server:&Res<AssetServer>
)-> Entity{
  let main_menu_entity = commands.spawn(
    (NodeBundle{
      style:get_main_menu_style(),
      //background_color: Color::RED.into(),
      ..default()
    },
    MainMenu,
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
                "dhcraftrs", 
                get_title_text_style(&asset_server),
              )
            ],
            alignment: TextAlignment::Center,
            ..default()
          },
          ..default()
        }
      );
      //image 2
      parent.spawn(ImageBundle{
        style: get_image_style(),
        image:asset_server.load("images/whiteblockblackline.png").into(),
        ..default()
      });
    });

    //new
    parent.spawn((
      ButtonBundle {
        style:get_button_style(),
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
      },
      NewGameButton
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "New Game", 
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
        style:get_button_style(),
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
      },
      PlayButton
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Play", 
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

    //online
    parent.spawn((
      ButtonBundle {
        style:get_button_style(),
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
      },
      OnlineButton
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Online", 
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

    //OPTIONS
    parent.spawn((
      ButtonBundle {
        style:get_button_style(),
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
      },
      OptionsButton
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Options", 
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

    //quit
    parent.spawn((
      ButtonBundle {
        style:get_button_style(),
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
      },
      QuitButton
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Quit", 
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

  main_menu_entity
}