/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

#[derive(Component)]
pub struct PLAYERMOVABLE;

//#[derive(Component)]
//pub struct Movable;

#[derive(Component)]
pub struct PlayerCamera;

#[allow(dead_code)]
pub struct PlayerInfo{
  id:String,
  idhash:String,
  name:String,
  is_dead:bool,
  is_spawn:bool,
}
