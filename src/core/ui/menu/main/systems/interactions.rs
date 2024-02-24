/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
*/

// https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file

use std::fs::File;

use bevy::app::AppExit;
use bevy::prelude::*;

use crate::core::api::AppState;
use crate::core::components::CameraMainMenu;
use crate::core::components::PlayerDataSlot;
use crate::core::components::PlayerInfo;
use crate::core::ui::menu::main::components::*;
use crate::core::ui::menu::main::styles::*;

// NEW, CREATE PLAYER
pub fn interact_with_new_button(
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<NewGameButton>)
  >,
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    match *interaction {
      Interaction::Pressed =>{
        *background_color = PRESSED_BUTTON_COLOR.into();
        app_state_next_state.set(AppState::CREATEPLAYERNAME);
      }
      Interaction::Hovered =>{
        *background_color = HOVERED_BUTTON_COLOR.into();
      }
      Interaction::None =>{
        *background_color = NORMAL_BUTTON_COLOR.into();
      }
    }
  }
}

// PLAY ACTION
pub fn interact_with_play_button(
  mut commands: Commands,
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<PlayButton>)
  >,
  mut app_state_next_state:ResMut<NextState<AppState>>,
  mut player_info: ResMut<PlayerInfo>,

  camera_main_menu_query:Query<Entity, With<CameraMainMenu>>,
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    match *interaction {
      Interaction::Pressed =>{
        *background_color = PRESSED_BUTTON_COLOR.into();

        let file = File::open("playerdata.json").unwrap();
        //let json: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
        let playerdata: PlayerDataSlot = serde_json::from_reader(file).expect("file should be proper JSON");
        println!("playerdataslot {:?}", playerdata);
        player_info.name = playerdata.name;

        //let json = Json::from_str(&data).unwrap();
        //let playerdataslot: PlayerDataSlot = serde_json::from_str(playerdata_file).expect("JSON was not well-formatted");
        //let playerdata_file = "playerdata.json";
        //let playerdataslot: PlayerDataSlot = serde_json::from_str(playerdata_file).expect("JSON was not well-formatted");
        //println!("playerdataslot {:?}", playerdataslot);

        //remove camera2d frin main menu
        if let Ok(camera_main_menu_entity) = camera_main_menu_query.get_single(){
          commands.entity(camera_main_menu_entity).despawn_recursive();
        }

        app_state_next_state.set(AppState::Game);
      }
      Interaction::Hovered =>{
        *background_color = HOVERED_BUTTON_COLOR.into();
      }
      Interaction::None =>{
        *background_color = NORMAL_BUTTON_COLOR.into();
      }
    }
  }
}

// ONLINE ACTION
pub fn interact_with_online_button(
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<OnlineButton>)
  >,
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    match *interaction {
      Interaction::Pressed =>{
        *background_color = PRESSED_BUTTON_COLOR.into();
        println!("NETWORK SET...");
        app_state_next_state.set(AppState::NETWORK);
      }
      Interaction::Hovered =>{
        *background_color = HOVERED_BUTTON_COLOR.into();
      }
      Interaction::None =>{
        *background_color = NORMAL_BUTTON_COLOR.into();
      }
    }
  }
}

// OPTIONS ACTION
pub fn interact_with_options_button(
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<OptionsButton>)
  >,
  mut app_state_next_state:ResMut<NextState<AppState>>,
  //mut ev_levelup: EventWriter<LevelUpEvent>,
  //query: Query<(Entity, &PlayerXp)>,
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    match *interaction {
      Interaction::Pressed =>{
        *background_color = PRESSED_BUTTON_COLOR.into();
        println!("SELECT OPTIONS");
        app_state_next_state.set(AppState::OPTIONS);
        
        //for (entity, xp) in query.iter() {
          //if xp.0 > 1000 {
            //ev_levelup.send(LevelUpEvent(entity));
          //}
        //}

      }
      Interaction::Hovered =>{
        *background_color = HOVERED_BUTTON_COLOR.into();
      }
      Interaction::None =>{
        *background_color = NORMAL_BUTTON_COLOR.into();
      }
    }
  }
}

//QUIT ACTION
pub fn interact_with_quit_button(
  mut app_exit_event_writer:EventWriter<AppExit>,
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<QuitButton>)
  >,
){
  if let Ok((interaction,mut background_color)) = button_query.get_single_mut(){
    match *interaction {
      Interaction::Pressed =>{
        *background_color = PRESSED_BUTTON_COLOR.into();
        app_exit_event_writer.send(AppExit);
      }
      Interaction::Hovered =>{
        *background_color = HOVERED_BUTTON_COLOR.into();
      }
      Interaction::None =>{
        *background_color = NORMAL_BUTTON_COLOR.into();
      }
    }
  }
}
