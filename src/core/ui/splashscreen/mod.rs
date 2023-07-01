/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

// https://bevy-cheatbook.github.io/features/time.html
use bevy::prelude::*;
//use std::time::Duration;

use self::systems::layout::{
  setup_splash_spawning, 
  splash_screen_time, 
  spawn_splash
};
pub mod systems;
pub mod components;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin{

  fn build(&self, app: &mut App){
    // set up entity
    app.add_startup_system(setup_splash_spawning);
    app.add_startup_system(spawn_splash);
    // timer to change sate.
    app.add_system(splash_screen_time);
    
  }

}
