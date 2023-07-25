/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//===============================================
// default style
//===============================================

use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR:Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR:Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR:Color = Color::rgb(0.35, 0.75, 0.35);

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
  TextStyle {
    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    font_size:32.0,
    color: Color::WHITE,
  }
}
