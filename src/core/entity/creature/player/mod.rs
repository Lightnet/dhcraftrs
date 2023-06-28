#![allow(unused_variables)]
/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
use bevy::prelude::*;

use crate::api::AppState;
//use bevy::input::mouse::MouseMotion;

//#[derive(Component)]
//pub struct Movable;

#[derive(Component)]
pub struct PLAYERMOVABLE;
#[derive(Component)]
pub struct PlayerCamera;

#[allow(dead_code)]
pub struct PlayerInfo{
  id:String,
  idhash:String,
  name:String,
  is_dead:bool,
  is_spawn:bool,
}

pub fn create_entity_player(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
){

  commands.spawn((
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },
    //PLAYERMOVABLE,
  ));

  // cube
  commands.spawn((
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::PINK,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    },
    PLAYERMOVABLE,
  )).with_children(| parent | {

    
    parent.spawn((
      Camera3dBundle {
        camera: Camera  { 
          order:1,
          //priority: 1 ,
          ..default()
        },
      transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..Default::default()
      },
      PlayerCamera
    ));
    

  });

  // camera
  //commands.spawn(Camera3dBundle {
    //transform: Transform::from_xyz(-20.0, 20.5, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    //..default()
  //});
}

pub fn player_movement(
  input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut query: Query<&mut Transform, With<PLAYERMOVABLE>>,
  //mut mouse_motion: EventReader<MouseMotion>,
  //mut camera_query: Query<(&mut Transform), With<Camera>>,
){
  for mut transform in &mut query {
    
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }
    if input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    transform.translation += time.delta_seconds() * 2.0 * direction;
  }
}

pub fn player_movement01(
  input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut query: Query<&mut Transform, With<PLAYERMOVABLE>>,
  //mut mouse_motion: EventReader<MouseMotion>,
  //mut camera_query: Query<(&mut Transform), With<Camera>>,
){
  for mut transform in &mut query {
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::Up) {
        direction.z += 1.0;
    }
    if input.pressed(KeyCode::Down) {
        direction.z -= 1.0;
    }
    if input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    transform.translation += time.delta_seconds() * 2.0 * direction;
  }
}

pub fn player_movement02(
  input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut query: Query<&mut Transform, With<PLAYERMOVABLE>>,
  //mut mouse_motion: EventReader<MouseMotion>,
  //mut camera_query: Query<(&mut Transform), With<Camera>>,
){
  let mut entity_transform = query.single_mut();

  //for mut transform in &mut query {
    //let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::Up) {
      //direction.z += 1.0;
      let direction = entity_transform.forward();
      entity_transform.translation += time.delta_seconds() * 1.0 * direction;
    }
    if input.pressed(KeyCode::Down) {
      let direction = entity_transform.back();
      entity_transform.translation += time.delta_seconds() * 1.0 * direction;
    }
    if input.pressed(KeyCode::Left) {
      //entity_transform.left()
      entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
        0., 1.0 * 0.01, 0.)
      );
    }
    if input.pressed(KeyCode::Right) {
      entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
        0., 1.0 * -0.01, 0.)
      );
    }

    //entity_transform.translation += time.delta_seconds() * 2.0 * direction;
  //}
}


pub struct CraftPlayerPlugin;

impl Plugin for CraftPlayerPlugin{

  fn build(&self, app: &mut App){
    //app.add_startup_system(create_entity_player);
    //app.add_system(player_movement02); //

    app.add_system(create_entity_player.in_schedule(OnEnter(AppState::InGame)));
    app.add_system(player_movement02.in_set(OnUpdate(AppState::InGame)));

    app.add_startup_system(set_app_state_game);
  }
}

fn set_app_state_game(
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  app_state_next_state.set(AppState::InGame);
}