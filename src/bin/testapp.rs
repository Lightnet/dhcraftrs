use bevy::{
  prelude::*, 
  //winit::WinitSettings,
  //tasks::IoTaskPool,
  //utils::Duration
};

use dhcraftrs::menu::MainMenuPlugin;
use dhcraftrs::AppState;

use dhcraftrs::systems::spawn_camera;

fn main() {

  println!("Test App!");

  App::new()
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    .add_plugin(MainMenuPlugin)

    .add_startup_system(spawn_camera)//need this for bevy ui to render
    .run();

}