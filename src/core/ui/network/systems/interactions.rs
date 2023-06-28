/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//use bevy::app::AppExit;
use bevy::prelude::*;

use crate::api::AppState;
use crate::core::ui::network::components::{HostNetworkButton, JoinNetworkButton};
//use crate::menu::components::*;
use crate::menu::styles::{
  HOVERED_BUTTON_COLOR,
  NORMAL_BUTTON_COLOR,
  PRESSED_BUTTON_COLOR
};

pub fn interact_with_host_button(
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<HostNetworkButton>)
  >,
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    match *interaction {
      Interaction::Clicked =>{
        *background_color = PRESSED_BUTTON_COLOR.into();
        app_state_next_state.set(AppState::SERVER);
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

pub fn interact_with_join_button(
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<JoinNetworkButton>)
  >,
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    match *interaction {
      Interaction::Clicked =>{
        *background_color = PRESSED_BUTTON_COLOR.into();
        app_state_next_state.set(AppState::CLIENT);
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