/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::app::AppExit;
use bevy::prelude::*;

use crate::api::AppState;
use crate::menu::components::*;
use crate::menu::styles::{
  HOVERED_BUTTON_COLOR,
  NORMAL_BUTTON_COLOR,
  PRESSED_BUTTON_COLOR
};

// NEW, CREATE PLAYER
pub fn interact_with_new_button(
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<NewButton>)
  >,
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    match *interaction {
      Interaction::Clicked =>{
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
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<PlayButton>)
  >,
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    match *interaction {
      Interaction::Clicked =>{
        *background_color = PRESSED_BUTTON_COLOR.into();
        app_state_next_state.set(AppState::InGame);
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
      Interaction::Clicked =>{
        *background_color = PRESSED_BUTTON_COLOR.into();
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
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    match *interaction {
      Interaction::Clicked =>{
        *background_color = PRESSED_BUTTON_COLOR.into();
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
      Interaction::Clicked =>{
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
