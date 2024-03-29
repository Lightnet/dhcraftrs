/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR:Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR:Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR:Color = Color::rgb(0.35, 0.75, 0.35);

pub fn get_button_style()-> Style{
  Style {
    justify_content:JustifyContent::Center,
    align_items:AlignItems::Center,
    width: Val::Px(200.0),
    height: Val::Px(80.0),
    margin: UiRect {
      bottom: Val::Px(2.0),
      left: Val::Px(2.0),
      right: Val::Px(2.0),
      top: Val::Px(2.0)
    },
    //size:Size::new(Val::Px(200.0),Val::Px(80.0)),
    ..default()
  }
}

pub fn get_main_menu_style()-> Style{
  Style {
    flex_direction:FlexDirection::Column,
    justify_content:JustifyContent::Center,
    align_items:AlignItems::Center,
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
    //gap:Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..default()
  }
}

pub fn get_title_style()-> Style{
  Style {
    flex_direction:FlexDirection::Row,
    justify_content:JustifyContent::Center,
    align_items:AlignItems::Center,
    width:Val::Px(300.0),
    height:Val::Px(120.0),
    //size: Size::new(Val::Px(300.0), Val::Px(120.0)),
    ..default()
  }
}

pub fn get_image_style()-> Style{
  Style {
    //size:Size::new(Val::Px(64.0), Val::Px(64.0)),
    width:Val::Px(64.0),
    //size: Size::width(Val::Px(64.0)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    margin:UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
  }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
  TextStyle {
    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    font_size:64.0,
    color: Color::WHITE,
  }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
  TextStyle {
    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    font_size:32.0,
    color: Color::WHITE,
  }
}