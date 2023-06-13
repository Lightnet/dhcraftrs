mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use self::systems::layout::{spawn_main_menu, despawn_main_menu};
use crate::api::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App){
    //app.add_startup_system(main_menu);
    println!("init main menu! plug in!");

    app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)));
    app.add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
  }
}

pub fn main_menu(){
  println!("You are on Main Menu.")
}