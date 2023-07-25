/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;

pub fn get_icon_slot_style() -> Style {
  Style {
    width: Val::Px(32.0),
    height: Val::Px(32.0),
    // center button
    //margin: UiRect::all(Val::Auto),
    margin: UiRect::all(Val::Px(2.0)),
    // horizontally center child text
    justify_content: JustifyContent::Center,
    // vertically center child text
    align_items: AlignItems::Center,
    ..default()
  }
}