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
  //tasks::IoTaskPool,
  //utils::Duration
};
use dhcraftrs::{
  api::AppState, 
  core::{
    console::ConsoleCraftPlugin, 
    ui::create_player::CreatePlayerPlugin
  }, plugins::DefaultCraftPlugin
};

//use dhcraftrs::test_print;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;

//#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
//enum AppState{
  //#[default]
  //MainMenu,
  //InGame,
//}

//#[derive(SystemSet, PartialEq, Eq, Clone, Hash, Debug)]
//enum PhysicsSet{
  //Movement,
  //CollisionDetection,
//}

fn main() {
  //test_print();
  App::new()
    .add_state::<AppState>()
    .add_plugins(DefaultPlugins)
    //.add_plugin(WorldInspectorPlugin)
    // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
    .insert_resource(WinitSettings::desktop_app())
    //.add_plugin(ConsolePlugin)
    .add_plugin(DefaultCraftPlugin)
    .add_plugin(ConsoleCraftPlugin)
    //.insert_resource(ConsoleConfiguration {
      // override config here
      //..Default::default()
    //})
    .add_plugin(CreatePlayerPlugin)
    .run();
}

// set up a simple 3D scene
/*
fn setup_simple_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });
  // cube
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
  });
  // light
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 1500.0,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..default()
  });
  // camera
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}
*/

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