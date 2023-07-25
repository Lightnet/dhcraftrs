/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

// https://bevy-cheatbook.github.io/features/time.html

pub mod systems;
pub mod components;
pub mod styles;

use bevy::prelude::*;
//use std::time::Duration;

use self::systems::layout::{
  setup_splash_spawning, 
  splash_screen_time, 
  spawn_splash
};

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin{

  fn build(&self, app: &mut App){
    // set up entity
    app.add_systems(Startup, setup_splash_spawning);
    app.add_systems(Startup,spawn_splash);
    // timer to change sate.
    app.add_systems(Update,splash_screen_time);
    
  }

}
