/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

#[allow(unused_imports)]
use crate::core::{
  ui::hud::hotbar::{
    components::{
      HOTBARID, 
      HOTBAR
    }, styles::*}, 
  api::AppState
};

pub fn interact_hot_bar_id_button(
  mut button_query:Query<
    (&Interaction, &mut BackgroundColor, &HOTBARID),
    (Changed<Interaction>, With<HOTBARID>)//need this? &HOTBARID
  >,
  //mut app_state_next_state:ResMut<NextState<AppState>>,
){
  //if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
  for (interaction, mut background_color, hotbarid) in button_query.iter_mut(){

    match *interaction {
      Interaction::Pressed =>{
        *background_color = PRESSED_ICON_BUTTON_COLOR.into();
        //app_state_next_state.set(AppState::CREATEPLAYERNAME);
        println!("HOTBARID {:?}", interaction );
        println!("background_color {:?}", background_color );
        println!("HOTBAR ID: {:?}", hotbarid );
        if hotbarid.0 == 2{
          println!("FOUND @!@");
        }
      }
      Interaction::Hovered =>{
        *background_color = HOVERED_ICON_BUTTON_COLOR.into();
      }
      Interaction::None =>{
        *background_color = NORMAL_ICON_BUTTON_COLOR.into();
      }
    }
  }
}