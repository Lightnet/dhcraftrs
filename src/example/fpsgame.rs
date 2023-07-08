/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

// Simple Testing
// https://bevyengine.org/learn/book/getting-started/ecs/
// https://github.com/bevyengine/bevy/blob/v0.10.1/examples/window/low_power.rs
#[allow(unused_imports)]
use bevy::{
  prelude::*,
  input::mouse::MouseMotion,
  window::{PresentMode, RequestRedraw, WindowResolution, WindowPlugin},
};
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_flycam::PlayerPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
  let mut app = App::new();
    //.add_plugins(DefaultPlugins)
    //app.insert_resource(ClearColor(Color::BLACK));
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        //width: WIDTH,
        //height: HEIGHT,
        resolution: WindowResolution::new(WIDTH, HEIGHT).with_scale_factor_override(1.0),
        title: "Bevy Game Test".to_string(),
        resizable: false,
        ..default()
      }),
      ..default()
    }));
    
    app.add_plugin(PlayerPlugin)
    //.add_plugin(WorldInspectorPlugin)
    .add_startup_system(setup)
    //.add_system(player_camera_controller)
    .run();
}

#[allow(dead_code)]
fn setup_camera(
  mut commands: Commands,
){
  // camera
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-20.0, 20.5, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(100.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  }).insert(Name::new("plane"));
  // cube
  //commands.spawn(PbrBundle {
      //mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      //material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
      //transform: Transform::from_xyz(0.0, 0.5, 0.0),
      //..default()
  //}).insert(Name::new("cube"));

  // cube
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size:32.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
}).insert(Name::new("cube"));

  // light
  commands.spawn(PointLightBundle {
      point_light: PointLight {
          intensity: 1500.0,
          shadows_enabled: true,
          ..default()
      },
      transform: Transform::from_xyz(4.0, 8.0, 4.0),
      ..default()
  }).insert(Name::new("light"));

  // camera
  //commands.spawn(Camera3dBundle {
    //transform: Transform::from_xyz(-20.0, 20.5, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    //..default()
  //});
}

#[allow(dead_code,unused_mut, unused_variables, unused_parens, non_snake_case)]
fn player_camera_controller(
  mut mouse_motion: EventReader<MouseMotion>,
  //windows: Res<Windows>,
  mut query: Query<(&mut Transform), With<Camera>>,
){
  //let window = get_primary_window_size(&windows);
  let mut camera_transform = query.single_mut();
  let mut yaw:f32 = 0.;
  let mut pitch:f32 = 0.;
  let mut TAU:f32 = 1.0;

  for ev in mouse_motion.iter() { //rewrite
    //camera_transform.rotate_y(ev.delta.x * TAU * -0.001);
    //camera_transform.rotate_x(ev.delta.y * TAU * -0.001);
    //camera_transform.rotate_z(0.0);

    //let mut yaw = Quat::from_euler(EulerRot::XYZ,
      //ev.delta.y * -0.001, ev.delta.x * -0.001, 0.);
    //yaw.y = 0.0;

    camera_transform
      .rotate(Quat::from_euler(EulerRot::XYZ,
        ev.delta.y * -0.001, ev.delta.x * -0.001, 0.));
  }

  /*
  for (mut transform) in query.iter_mut() {
    for ev in motion.iter() {
      yaw = ev.delta.x * 0.01 ; // window.x * player.sensitivity;
      pitch = ev.delta.y * -0.01; // / window.y * player.sensitivity;
    }
    transform.rotate(Quat::from_rotation_y(-yaw) * Quat::from_rotation_x(pitch) * Quat::from_rotation_z(0.));
  }
  */
}