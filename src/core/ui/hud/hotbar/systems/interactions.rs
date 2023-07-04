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
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<HOTBARID>)
  >,
  //mut app_state_next_state:ResMut<NextState<AppState>>,
){
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
    match *interaction {
      Interaction::Clicked =>{
        *background_color = PRESSED_ICON_BUTTON_COLOR.into();
        //app_state_next_state.set(AppState::CREATEPLAYERNAME);
        print!("HOTBAR ...", );
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