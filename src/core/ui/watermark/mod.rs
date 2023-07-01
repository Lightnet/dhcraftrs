/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
/*
  Information:
    Just for prototype to add watermark in case of stolen content that not made by the user.
 */
// https://bevy-cheatbook.github.io/features/time.html

mod components;
mod styles;
mod systems;

//use std::time::Duration;
use bevy::prelude::*;

use crate::core::api::AppState;

use self::systems::layout::spawn_water_mark;
use self::systems::layout::despawn_water_mark;

#[derive(Component)]
pub struct WaterMark;

#[derive(Resource)]
pub struct WaterMarkConfig;

pub struct WaterMarkPlugin;

impl Plugin for WaterMarkPlugin{

  fn build(&self, app: &mut App){
    app.add_startup_system(setup_water_mark);
    //app.add_system(setup_water_mark)
    app.add_system(spawn_water_mark.in_schedule(OnEnter(AppState::MainMenu)));
    app.add_system(despawn_water_mark.in_schedule(OnExit(AppState::MainMenu)));
  }
}

fn setup_water_mark(
  mut commands: Commands,
){
  commands.insert_resource(WaterMarkConfig)
}
//todolist : remove?
#[allow(dead_code)]
fn spawn_splash(
  mut _commands: Commands,
  mut _config: ResMut<WaterMarkConfig>,
){
  //need to check if exist
}



