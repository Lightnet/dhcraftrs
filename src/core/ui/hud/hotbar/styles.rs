/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

pub const NORMAL_ICON_BUTTON_COLOR:Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_ICON_BUTTON_COLOR:Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_ICON_BUTTON_COLOR:Color = Color::rgb(0.35, 0.75, 0.35);

pub const HOT_BAR_STYLE:Style = Style {
  position_type: PositionType::Absolute,
  //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
  position: UiRect {
    bottom: Val::Px(4.0),
    //top: Val::Px(5.0),
    //right: Val::Px(5.0),
    //right: Val::Percent(50.0),
    //left: Val::Percent(50.0),
    ..UiRect::DEFAULT
  },
  //align_self:AlignSelf::FlexEnd,
  align_self:AlignSelf::Center,
  flex_direction:FlexDirection::Row,
  justify_content:JustifyContent::Center,
  //justify_content:JustifyContent::FlexEnd,
  align_items:AlignItems::Center,
  size: Size::width(Val::Percent(100.0)),
  //size:Size::new(Val::Percent(100.0), Val::Percent(100.0)),
  //gap:Size::new(Val::Px(8.0), Val::Px(8.0)),
  ..Style::DEFAULT
};

pub const ICON_HOT_BAR_STYLE01:Style = Style {
  justify_content:JustifyContent::Center,
  align_items:AlignItems::Center,
  margin: UiRect::all(Val::Px(2.0)),
  align_self: AlignSelf::FlexEnd,
  gap:Size::new(Val::Px(8.0), Val::Px(8.0)),
  size:Size::new(Val::Px(64.0),Val::Px(64.0)),
  ..Style::DEFAULT
};

pub const ICON_HOT_BAR_STYLE:Style = Style {
  justify_content:JustifyContent::Center,
  align_items:AlignItems::Center,
  size:Size::new(Val::Px(64.0),Val::Px(64.0)),
  ..Style::DEFAULT
};
