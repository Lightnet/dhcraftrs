
/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
use bevy::prelude::*;

//use bevy::diagnostic::{
  //Diagnostics, 
  //FrameTimeDiagnosticsPlugin
//};
 
use crate::api::AppState;
use crate::core::ui::create_player::components::{CREATEPLAYERNAMEBUTTON, PlayerNameText};
use crate::menu::styles::{
  HOVERED_BUTTON_COLOR,
  NORMAL_BUTTON_COLOR,
  PRESSED_BUTTON_COLOR
};
 
pub fn interact_with_new_button(
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<CREATEPLAYERNAMEBUTTON>)
  >,
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    match *interaction {
      Interaction::Clicked =>{
        *background_color = PRESSED_BUTTON_COLOR.into();
        app_state_next_state.set(AppState::LoadingGame);
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

pub fn player_name_text_update(
  //diagnostics: Res<Diagnostics>,
  mut query: Query<&mut Text, With<PlayerNameText>>,
  mut evr_char: EventReader<ReceivedCharacter>,
  kbd: Res<Input<KeyCode>>,
  mut string: Local<String>,
) {
  if kbd.just_pressed(KeyCode::Return) {
    println!("Text input: {}", &*string);
    string.clear();
  }
  if kbd.just_pressed(KeyCode::Back) {
    string.pop();
  }
  for ev in evr_char.iter() {
    // ignore control (special) characters
    if !ev.char.is_control() {
      string.push(ev.char);
      println!("CHAR: {}",ev.char);
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
      text.sections[0].value = format!("{}", string.to_string());
  }
}