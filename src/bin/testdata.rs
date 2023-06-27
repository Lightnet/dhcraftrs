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
use moonshine_save::prelude::*;

// Saved components must derive `Reflect`
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Player;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Level(u32);

#[derive(Bundle)]
struct PlayerBundle {
  player: Player,
  level: Level,
  name: Name,
  save: Save,
}

#[derive(Bundle)]
struct PlayerInfoBundle {
  level:Level,
  save: Save,
}

/// A resource which is used to invoke the save system.
#[derive(Resource)]
struct SaveRequest;

/// A resource which is used to invoke the load system.
#[derive(Resource)]
struct LoadRequest;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    // Usually, both are needed:
    .add_plugin(LoadPlugin)
    .add_plugin(SavePlugin)
    // structs
    .register_type::<Player>()
    .register_type::<Level>()
    //.register_type::<PlayerInfoBundle>()
    //.add_system(save_into_file("saved.ron"))//not this it will loop
    //.add_startup_system(setup)
    //.add_startup_system(setup)// do this
    //.add_system(setup.on_startup())// do this

    .add_systems(
      (save_into_file("saved.ron"), remove_save_request)
        .chain()
        .distributive_run_if(should_save),
    )// do this

    .add_system(input_key)// do this
    //.add_system(save_into_file("saved.ron").on_startup())// nope does not save
    //.add_startup_system(save_into_file("saved.ron"))// nope does not save
    //.add_system(save_into_file("saved.ron"))//not this it will loop
    .run();
}

#[allow(unused_variables, dead_code, unused_parens, unused_mut)]
fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){

  println!("setup...");

  commands.spawn(
  (
    //PbrBundle {
      //mesh: meshes.add(shape::Plane::from_size(5.0).into()),
      //material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      //..default()
    //},
      PlayerInfoBundle {
        level:Level(1),
        save: Save,
      }
      //Level(1),
  )
  );

  // camera
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}

/// Returns true if the save systems should be invoked.
fn should_save(request: Option<Res<SaveRequest>>) -> bool {
  request.is_some()
}

fn remove_save_request(world: &mut World) {
  world.remove_resource::<SaveRequest>().unwrap();
}

/// Returns true if the load systems should be invoked.
fn should_load(request: Option<Res<LoadRequest>>) -> bool {
  request.is_some()
}

fn remove_load_request(world: &mut World) {
  world.remove_resource::<LoadRequest>().unwrap();
}

fn input_key(
  mut commands: Commands,
  input: Res<Input<KeyCode>>,
  time: Res<Time>,
){
  if input.pressed(KeyCode::Up) {
    println!("UP");
    //save_into_file("saved.ron");//nope
    commands.insert_resource(SaveRequest);
  }
}