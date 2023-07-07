/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::Component;

#[derive(Component)]
pub struct CREATEPLAYERNAME;

#[derive(Component)]
pub struct CREATEPLAYERNAMEBUTTON;

// https://bevyengine.org/examples/ui/text/
// A unit struct to help identify the color-changing Text component
#[derive(Component)]
pub struct PlayerNameText;

#[derive(Component)]
pub struct BackButton;
