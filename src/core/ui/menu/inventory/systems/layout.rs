/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;

use crate::core::{
  ui::menu::inventory::{
    components::InventoryUIRoot, 
    styles::get_icon_slot_style
  }, 
  systems::get_text_style,
  styles::*
};

pub fn spawn_inventory_menu(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
  println!("init menu...");
  build_inventory_menu(&mut commands, &asset_server);

}

pub fn build_inventory_menu(
  commands: &mut Commands,
  asset_server:&Res<AssetServer>
){
  commands.spawn(NodeBundle{
    style: Style {
      //width: Val::Percent(100.0),
      //height: Val::Percent(100.0),
      flex_direction:FlexDirection::Column,
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      ..default()
    },
    ..default()
  })
  .insert(InventoryUIRoot)
  .with_children(|parent|{

    parent.spawn(NodeBundle{
      style: Style {
        //width: Val::Percent(100.0),
        //height: Val::Percent(100.0),
        flex_direction:FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    }).with_children(|parent|{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Inventory", 
                get_text_style(&asset_server),
              )
            ],
            alignment: TextAlignment::Center,
            ..default()
          },
          ..default()
        }
      );
    });
    //row 1 slots
    parent.spawn(NodeBundle{
      style: Style {
        //width: Val::Percent(100.0),
        //height: Val::Percent(100.0),
        flex_direction:FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    }).with_children(|mut parent|{

      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);

    });
    //row 2 slots
    parent.spawn(NodeBundle{
      style: Style {
        //width: Val::Percent(100.0),
        //height: Val::Percent(100.0),
        flex_direction:FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    }).with_children(|mut parent|{

      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);

    });
    //row 3 slots
    parent.spawn(NodeBundle{
      style: Style {
        //width: Val::Percent(100.0),
        //height: Val::Percent(100.0),
        flex_direction:FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    }).with_children(|mut parent|{

      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
    });

    //row 4 slots
    parent.spawn(NodeBundle{
      style: Style {
        //width: Val::Percent(100.0),
        //height: Val::Percent(100.0),
        flex_direction:FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    }).with_children(|mut parent|{

      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
    });

    //space
    parent.spawn(NodeBundle{
      style: Style {
        //width: Val::Percent(100.0),
        //height: Val::Percent(100.0),
        //height: Val::Px(32.0),
        flex_direction:FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    }).with_children(|parent|{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Hot Bars", 
                get_text_style(&asset_server),
              )
            ],
            alignment: TextAlignment::Center,
            ..default()
          },
          ..default()
        }
      );
    });


    //row 5 slots
    parent.spawn(NodeBundle{
      style: Style {
        //width: Val::Percent(100.0),
        //height: Val::Percent(100.0),
        flex_direction:FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    }).with_children(|mut parent|{

      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
      create_icon_inventory(&mut parent);
    });


  });
}


fn create_icon_inventory(
  commands:&mut ChildBuilder,
){
  commands.spawn(ButtonBundle {
    style: get_icon_slot_style(),
    background_color: NORMAL_BUTTON_COLOR.into(),
    ..default()
  });
}

pub fn despawn_inventory_menu(
  mut commands: Commands,
  menu_query:Query<Entity, With<InventoryUIRoot>>,
){
  if let Ok(menu_entity) = menu_query.get_single(){
    commands.entity(menu_entity).despawn_recursive();
  }
}

pub fn build_inventory(){

}