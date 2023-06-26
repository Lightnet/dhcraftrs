/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//===============================================
//
//===============================================
// https://bevy-cheatbook.github.io/features/time.html
use bevy::prelude::*;
//use std::time::Duration;

#[derive(Component)]
pub struct WaterMark;

#[derive(Resource)]
pub struct WaterMarkConfig;

pub struct WaterMarkPlugin;

impl Plugin for WaterMarkPlugin{

  fn build(&self, app: &mut App){
    app.add_startup_system(setup_water_mark);
    app.add_system(spawn_splash);
  }

}

fn setup_water_mark(
  mut commands: Commands,
){
  commands.insert_resource(WaterMarkConfig)
}

fn spawn_splash(
  mut _commands: Commands,
  mut _config: ResMut<WaterMarkConfig>,
){
  //need to check if exist

}



