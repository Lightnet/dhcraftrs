/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

use crate::{
  core::{ui::hud::hotbar::{
    styles::*, 
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
        style:ICON_HOT_BAR_STYLE01,
        background_color: NORMAL_ICON_BUTTON_COLOR.into(),
        ..default()
      },
      HOTBARID(1)
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

    //HOTBAR 2
    parent.spawn((
      ButtonBundle {
        style:ICON_HOT_BAR_STYLE01,
        background_color: NORMAL_ICON_BUTTON_COLOR.into(),
        ..default()
      },
      HOTBARID(2)
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

    //HOTBAR 3
    parent.spawn((
      ButtonBundle {
        style:ICON_HOT_BAR_STYLE01,
        background_color: NORMAL_ICON_BUTTON_COLOR.into(),
        ..default()
      },
      HOTBARID(3)
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "No3", 
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

    //HOTBAR 4
    parent.spawn((
      ButtonBundle {
        style:ICON_HOT_BAR_STYLE01,
        background_color: NORMAL_ICON_BUTTON_COLOR.into(),
        ..default()
      },
      HOTBARID(4)
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "No4", 
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

    //HOTBAR 5
    parent.spawn((
      ButtonBundle {
        style:ICON_HOT_BAR_STYLE01,
        background_color: NORMAL_ICON_BUTTON_COLOR.into(),
        ..default()
      },
      HOTBARID(5)
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "No5", 
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

    //HOTBAR 6
    parent.spawn((
      ButtonBundle {
        style:ICON_HOT_BAR_STYLE01,
        background_color: NORMAL_ICON_BUTTON_COLOR.into(),
        ..default()
      },
      HOTBARID(6)
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "No6", 
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

    //HOTBAR 7
    parent.spawn((
      ButtonBundle {
        style:ICON_HOT_BAR_STYLE01,
        background_color: NORMAL_ICON_BUTTON_COLOR.into(),
        ..default()
      },
      HOTBARID(7)
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "No7", 
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

    //HOTBAR 8
    parent.spawn((
      ButtonBundle {
        style:ICON_HOT_BAR_STYLE01,
        background_color: NORMAL_ICON_BUTTON_COLOR.into(),
        ..default()
      },
      HOTBARID(8)
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "No8", 
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

    //HOTBAR 9
    parent.spawn((
      ButtonBundle {
        style:ICON_HOT_BAR_STYLE01,
        background_color: NORMAL_ICON_BUTTON_COLOR.into(),
        ..default()
      },
      HOTBARID(9)
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "No9", 
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

    //HOTBAR 0
    parent.spawn((
      ButtonBundle {
        style:ICON_HOT_BAR_STYLE01,
        background_color: NORMAL_ICON_BUTTON_COLOR.into(),
        ..default()
      },
      HOTBARID(0)
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "No0", 
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