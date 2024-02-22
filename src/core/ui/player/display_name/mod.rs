/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
*/

use bevy::prelude::*;

use crate::core::{api::AppState, ui::menu::main::styles::{
  //get_main_menu_style, 
  get_title_text_style
}, components::PlayerInfo};

#[derive(Component)]
pub struct UIPLAYERNAME;

#[derive(Component)]
pub struct PLAYERNAMETEXT;

pub fn spawn_create_player_name(
  mut commands: Commands,
  asset_server:Res<AssetServer>,
  player_info: Res<PlayerInfo>,
){
  let _create_player_menu_entity = build_create_player_name(&mut commands, &asset_server, &player_info);
}

pub fn despawn_create_player_name(
  mut commands: Commands,
  main_menu_query:Query<Entity, With<UIPLAYERNAME>>,
){
  if let Ok(main_menu_entity) = main_menu_query.get_single(){
    commands.entity(main_menu_entity).despawn_recursive();
  }
}

pub fn build_create_player_name(
  commands: &mut Commands,
  asset_server:&Res<AssetServer>,
  player_info: &Res<PlayerInfo>
)-> Entity{
  let player_name_text_entity = commands.spawn(
    (NodeBundle{
      style:get_player_name_text_style(),
      //background_color: Color::RED.into(),
      ..default()
    },
    UIPLAYERNAME,
    )
  ).with_children(|parent |{
    //text
    parent.spawn((
      TextBundle{
        text: Text{
          sections: vec![
            TextSection::new(
              player_info.name.to_string(),
              //"Player",
              get_title_text_style(&asset_server),
            )
          ],
          //alignment: TextAlignment::Center,
          ..default()
        },
        ..default()
      },
      PLAYERNAMETEXT
    ));

  })
  .id();

  player_name_text_entity
}

pub fn get_player_name_text_style()->Style{
  Style {
    position_type: PositionType::Absolute,
    //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    top: Val::Px(4.0),
    //bottom: Val::Px(4.0),
    //align_self:AlignSelf::FlexEnd,
    align_self:AlignSelf::Center,
    flex_direction:FlexDirection::Row,
    justify_content:JustifyContent::Center,
    align_items:AlignItems::Center,
    width: Val::Percent(100.0),
    //gap:Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
  }
}


pub struct DisplayPlayerNameTextPlugin;

impl Plugin for DisplayPlayerNameTextPlugin {
  fn build(&self, app: &mut App){
    //create ui
    app.add_systems(OnEnter(AppState::Game), spawn_create_player_name);
    //remove ui
    app.add_systems(OnExit(AppState::Game), despawn_create_player_name);
    
    //button event interact //input text
    //app.add_systems(Update, 
      //(
        
      //).run_if(in_state(AppState::Game))
    //);
  }
}