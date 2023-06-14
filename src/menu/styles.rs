use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR:Color = Color::rgb(0.15, 0.15, 0.15);
pub const BUTTON_STYLE:Style = Style {
  justify_content:JustifyContent::Center,
  align_items:AlignItems::Center,
  size:Size::new(Val::Px(200.0),Val::Px(80.0)),
  ..Style::DEFAULT
};