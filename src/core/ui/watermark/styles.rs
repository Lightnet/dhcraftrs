/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

pub const WATER_MARK_STYLE:Style = Style {
  position_type: PositionType::Absolute,
  position: UiRect {
    top: Val::Px(0.),
    ..UiRect::DEFAULT
  },
  flex_direction:FlexDirection::Column,
  //justify_content:JustifyContent::Center,
  justify_content:JustifyContent::FlexStart,
  //align_items:AlignItems::Center,
  align_items:AlignItems::Center,
  align_self:AlignSelf::Center,
  size:Size::new(Val::Percent(100.0), Val::Percent(100.0)),
  gap:Size::new(Val::Px(8.0), Val::Px(8.0)),
  ..Style::DEFAULT
};