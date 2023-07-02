#[warn(unused_imports)]

/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//use std::{fs::File, io::Read};
//use std::io::Write;

use bevy::{
  prelude::*, 
  winit::WinitSettings,
};
use dhcraftrs::core::window::set_window_icon;
#[allow(dead_code)]
use dhcraftrs::plugins::DefaultCraftPlugin;

//use bevy_inspector_egui::quick::WorldInspectorPlugin;

//#[derive(SystemSet, PartialEq, Eq, Clone, Hash, Debug)]
//enum PhysicsSet{
  //Movement,
  //CollisionDetection,
//}

fn main() {
  //test_print();
  println!("GAME APPLICATION");
  App::new()
    //.add_plugins(DefaultPlugins)
    //.add_plugin(WorldInspectorPlugin)
    // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
    //.insert_resource(WinitSettings::desktop_app())//lag input?
    //.add_plugin(ConsolePlugin)
    
    .add_plugin(DefaultCraftPlugin)
    //.add_startup_system(set_window_icon)
    
    //.add_plugin(ConsoleCraftPlugin)
    //.insert_resource(ConsoleConfiguration {
      // override config here
      //..Default::default()
    //})
    .run();
}

/*
// The new, updated scene data will be saved here so that you can see the changes
const NEW_SCENE_FILE_PATH: &str = "scenes/test_scene.scn.ron";
// https://github.com/bevyengine/bevy/blob/main/examples/scene/scene.rs
fn test_save_file(
  mut commands: Commands,
  server: Res<AssetServer>
){
  //let mut file = File::create("assets/scenes/test_scene.scn.ron").unwrap();

  //file.write_all(b"Hello, world!").unwrap();
  //file.write(b"Hello, world!").unwrap();

  //let mut file = File::create("assets/scenes/test_scene.scn.ron")?;
  //file.write_all(b"Hello, world!")?;

  #[cfg(not(target_arch = "wasm32"))]
  IoTaskPool::get()
      .spawn(async move {
          // Write the scene RON data to file
          File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
              .and_then(|mut file| file.write(b"helloworld"  ))
              .expect("Error while writing scene to file");
      })
      .detach();
}

fn test_load_file(
  mut commands: Commands,
  server: Res<AssetServer>
){
  #[cfg(not(target_arch = "wasm32"))]
  IoTaskPool::get()
      .spawn(async move {
          // Write the scene RON data to file
          let mut contents = String::new();
          File::open(format!("assets/{NEW_SCENE_FILE_PATH}"))
              .and_then(|mut file|  
                file.read_to_string(&mut contents)
              )
              .expect("Error while writing scene to file");
          println!("{}", contents);
      })
      .detach();
}
*/