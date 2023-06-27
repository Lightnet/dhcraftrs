/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

// https://crates.io/crates/moonshine-save
// https://github.com/Zeenobit/moonshine_save/blob/HEAD/examples/army.rs

use bevy::prelude::*;
use bevy_pkv::PkvStore;

// https://crates.io/crates/bevy_pkv

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(PkvStore::new("dhcraftrs", "player")) // Users\<username>\AppData\Roaming\<dhcraftrs>
    .add_startup_system(setup_pkv)
    .run();
}

fn setup_pkv(mut pkv: ResMut<PkvStore>) {
  if let Ok(username) = pkv.get::<String>("username") {
    info!("Welcome back {username}");
    println!("Welcome back {username}");
  } else {
    println!("create user!!");
    pkv.set_string("username", "alice")
        .expect("failed to store username");

    // alternatively, using the slightly less efficient generic api:
    pkv.set("username", &"alice".to_string())
        .expect("failed to store username");
  }
}

fn setup(
  mut commands: Commands,
  mut _meshes: ResMut<Assets<Mesh>>,
  mut _materials: ResMut<Assets<StandardMaterial>>,
){

  //println!("setup...");
  // camera
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}

#[allow(unused_mut)]
fn input_key(
  mut commands: Commands,
  input: Res<Input<KeyCode>>,
  time: Res<Time>,
){
  if input.pressed(KeyCode::Up) {
    println!("UP");
    //save_into_file("saved.ron");//nope
    //commands.insert_resource(SaveRequest);
  }
}