/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//===============================================
//
//===============================================

use bevy::prelude::*;

use crate::{core::{ui::watermark::{styles::WATER_MARK_STYLE, components::WaterMark}, styles::get_button_text_style}};

pub fn spawn_water_mark(
  mut commands: Commands,
  asset_server:Res<AssetServer>,
){
  //println!("WATERMARK ADD?");
  let _water_mark_entity = build_water_mark(&mut commands, &asset_server);
}

pub fn despawn_water_mark(
  mut commands: Commands,
  water_mark_query:Query<Entity, With<WaterMark>>,
){
  if let Ok(water_mark_entity) = water_mark_query.get_single(){
    commands.entity(water_mark_entity).despawn_recursive();
  }
}

pub fn build_water_mark(
  commands: &mut Commands,
  asset_server:&Res<AssetServer>
)-> Entity{
  let watermark_entity = commands.spawn(
    (NodeBundle{
      style:WATER_MARK_STYLE,
      //background_color: Color::RED.into(),
      ..default()
    },
    WaterMark,
    )
  ).with_children(|parent |{
    parent.spawn(
      TextBundle{
        text: Text{
          sections: vec![
            TextSection::new(
              "DHCraftrs Alpha 0.0.0", 
              get_button_text_style(&asset_server),
            )
          ],
          alignment: TextAlignment::Center,
          ..default()
        },
        ..default()
      }
    );
  }).id();

  watermark_entity
}