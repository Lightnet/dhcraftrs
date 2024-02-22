/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

// https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs
// https://stackoverflow.com/questions/69449293/how-to-write-a-vector-to-a-json-file
// https://onelinerhub.com/rust/how-to-write-struct-to-json-file-in-rust


use bevy::prelude::*;
use serde_json::json;
//use bevy_pkv::PkvStore;
use crate::core::api::AppState;
use crate::core::components::PlayerInfo;
use crate::core::ui::menu::create_player::components::{CREATEPLAYERNAMEBUTTON, PlayerNameText, BackButton};
use crate::core::ui::menu::main::styles::{PRESSED_BUTTON_COLOR, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR};

use std::fs::File;
//use std::io::BufWriter;

pub fn interact_button_create_player(
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<CREATEPLAYERNAMEBUTTON>)
  >,
  mut app_state_next_state:ResMut<NextState<AppState>>,
  player_info: Res<PlayerInfo>,
  //mut pkv: ResMut<PkvStore>,
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    println!("new button player create");
    match *interaction {
      Interaction::Pressed =>{
        *background_color = PRESSED_BUTTON_COLOR.into();
        println!("Player Name: {}", player_info.name );

        //pkv.set_string("username", player_info.name.as_str() )
        //.expect("failed to store username");
        let player_data = json!({
          "name": player_info.name,
          "idhash":"00"
        });

        println!("DATA: {}", player_data.to_string());

        let mut file = File::create("playerdata.json").unwrap();
        //let mut writer = BufWriter::new(file);
        //serde_json::to_writer(&mut writer, &player_data).unwrap();
        serde_json::to_writer_pretty(&mut file, &player_data).unwrap();


        //need to check blank incase of player name string is empty later...
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

pub fn interact_button_back(
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<BackButton>)
  >,
  mut app_state_next_state:ResMut<NextState<AppState>>,
  player_info: Res<PlayerInfo>,
  //mut pkv: ResMut<PkvStore>,
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    println!("new button player create");
    match *interaction {
      Interaction::Pressed =>{
        *background_color = PRESSED_BUTTON_COLOR.into();
        println!("Player Name: {}", player_info.name );

        //pkv.set_string("username", player_info.name.as_str() )
        //.expect("failed to store username");

        //need to check blank incase of player name string is empty later...
        app_state_next_state.set(AppState::MainMenu);
        
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

/*
pub fn player_name_text_update(
  //diagnostics: Res<Diagnostics>,
  mut query: Query<&mut Text, With<PlayerNameText>>,
  mut evr_char: EventReader<ReceivedCharacter>,
  kbd: Res<ButtonInput<KeyCode>>,
  //mut string: Local<String>,
  mut player_info: ResMut<PlayerInfo>,
) {

  //println!("player name: {:?}", player_info.name);
  if kbd.just_pressed(KeyCode::Return) {//Return
    //println!("Text input: {}", &*string);
    //string.clear();
    //println!("Text input: {}", &* player_info.name);
    //player_info.name = string.to_string();
    //player_info.name = "test".into();
  }
  if kbd.just_pressed(KeyCode::KeyBack) {
    //string.pop();
    player_info.name.pop();
  }
  for ev in evr_char.read() {
    // ignore control (special) characters
    if !ev.char.is_control() {
      //string.push(ev.char);
      println!("CHAR: {}",ev.char);
      player_info.name.push(ev.char);
      //player_info.name;
      //player_info.name = player_info.name
    }
  }

  for mut text in &mut query {
      //if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
          //if let Some(value) = fps.smoothed() {
              // Update the value of the second section
              //text.sections[1].value = format!("{value:.2}");
          //}
      //}
      //text.sections[0].value = format!("test");
      //text.sections[0].value = format!("{}", string.to_string());
      text.sections[0].value = format!("{}", player_info.name.to_string());
  }
}
*/