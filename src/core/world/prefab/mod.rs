/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

pub mod base;
use bevy::prelude::*;

use crate::core::api::AppState;

use self::base::basic_scene_test;

pub struct WorldBasicPlugin;

impl Plugin for WorldBasicPlugin{
  fn build(&self, app: &mut App){
    //test loading entity
    app.add_system(basic_scene_test.in_schedule(OnEnter(AppState::Game)));
  }
}