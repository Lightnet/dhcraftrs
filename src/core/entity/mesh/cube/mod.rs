/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//use std::ops::Add;

use bevy::prelude::*;
//use bevy_mod_raycast::RaycastMesh;
//use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;
#[allow(unused_imports)]
use crate::core::{components::PlaceHolder, entity::creature::player::components::{PLAYERMOVABLE, PlayerTool}};
//use bevy_eventlistener::prelude::*;

//basic set up cube test
pub fn setup_create_cube(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  commands.spawn(PbrBundle {
    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
    ..default()
  }).insert(Name::new("cube"));
}

pub fn setup_ph_cube(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  commands.spawn(PbrBundle {
    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    //material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    material: materials.add(StandardMaterial {
      base_color: Color::rgba(0.9, 0.9, 0.9, 0.5),
      alpha_mode: AlphaMode::Blend,
      ..default()
    }),
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
    ..default()
  })
  .insert(PlaceHolder)
  .insert(Name::new("placeholder"));
}
/*
material: materials.add(StandardMaterial {
  //base_color: Color::GREEN,
  //base_color: Color::rgba(0.2, 0.7, 0.1, 0.0),
  base_color: Color::rgba(0.9, 0.9, 0.9, 0.5),
  //alpha_mode:  AlphaMode::Mask(0.8),
  alpha_mode: AlphaMode::Blend,
  //unlit: true,
  //cull_mode: None,
  ..default()
}),
*/

pub fn place_holder_update(
  
){

}

// System
pub fn create_entity_cube(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  
  commands
    .spawn(
      PbrBundle{
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(StandardMaterial {
          base_color: Color::rgba(0.6, 0.6, 0.6, 0.5),
          alpha_mode: AlphaMode::Blend,
          ..default()
        }),
        //transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
      }
    );
}
// System
pub fn create_entity_cube_pick(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  
  commands
    .spawn((
      PbrBundle{
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::WHITE),
        //material: materials.add(StandardMaterial {
          //base_color: Color::rgba(0.1, 0.6, 0.6, 0.5),
          //alpha_mode: AlphaMode::Blend,
          //..default()
        //}),
        //transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
      },
      //PickableBundle::default(),
      //RaycastPickTarget::default(),
      //#[allow(dead_code,unused_variables)]
      //OnPointer::<Click>::target_commands_mut(|click, target_commands| {
        //if click.target != click.listener && click.button == PointerButton::Secondary {
            //target_commands.despawn();
        //}
        //println!("Right Click============================");
      //}),
    ))
    //.insert(PickableBundle::default())
    //.insert( RaycastPickTarget::default())
    //.insert(OnPointer::<Click>::target_commands_mut(|click, target_commands| {
      //if click.target != click.listener() && click.button == PointerButton::Secondary {
        //target_commands.despawn();
        //println!("CLICKED...");
        //target_commands.despawn();
      //}
    //}))

    //.insert(OnPointer::<Click>::run_callback(my_pick_click))
    ;
}

// System
#[allow(dead_code,unused_variables)]
pub fn create_entity_cube_physics(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  //buttons: Res<Input<MouseButton>>,
){
  create_cube_physics(&mut commands, &mut meshes, &mut materials,  Vec3::new(0., 0., 0.));
  //create_cube_physics(&mut commands, &mut meshes, &mut materials,  Vec3::new(0., 0., -3.));
}

pub fn create_raycast_cube_physics(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  //buttons: Res<Input<MouseButton>>,
){
  commands
  .spawn((
    PbrBundle{
      mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
      material: materials.add(Color::BLACK),
      ..default()
    },
  ))
  .insert(Collider::cuboid(0.5, 0.5, 0.5))
  .insert(TransformBundle::from(Transform::from_xyz(0., 0., 0.)))
  //.insert(RaycastMesh::<MyRaycastSet>::default())
  ;
}

// no System
#[allow(dead_code,unused_variables)]
pub fn create_cube_physics(
  commands: &mut Commands,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<StandardMaterial>>,
  //buttons: &Res<Input<MouseButton>>,
  position: Vec3
){
  commands
  .spawn((
    PbrBundle{
      mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
      material: materials.add(Color::BLACK),
      ..default()
    },
    //#[allow(dead_code,unused_variables)]
    //OnPointer::<Click>::target_commands_mut(|click, target_commands| {
      //if click.target != click.listener && click.button == PointerButton::Secondary {
      //if click.button == PointerButton::Secondary {
        
        //println!("[[[    Event: {:?}", click.pointer_event);
        //println!("[[[    listener: {:?}", click.listener);
        //println!("[[[    target: {:?}", click.target);
        //println!("[[[    button: {:?}", click.button);
        //println!("[[[    hit: {:?}", click.hit);
        //println!("[[[    hit: {:?}", click.pointer_event.hit);
        //target_commands.despawn();
        //println!("Right Click============================");
      //}
    //}),
  ))
  .insert(Collider::cuboid(0.5, 0.5, 0.5))
  //.insert(PickableBundle::default())
  //.insert( RaycastPickTarget::default())
  //.insert(OnPointer::<Click>::target_commands_mut(|click, target_commands| {
      //if click.target != click.listener && click.button == PointerButton::Secondary {
        //target_commands.despawn();
      //}
      //println!("Right Click===[[[ create_entity_cube_physics ]]]===");
  //}))
  //.insert(OnPointer::<Click>::run_callback(click_build_block))
  .insert(TransformBundle::from(Transform::from_xyz(position.x, position.y, position.z)))
  //.insert(OnPointer::<Move>::run_callback(update_place_holder_item))
  ;
}


// BUILD block here?
// https://bevy-cheatbook.github.io/input/mouse.html#mouse-buttons
// trigger once
// System
/*
#[allow(unused_mut, unused_variables)]
fn click_build_block(
  // The first parameter is always the `ListenedEvent`, passed in by the event listening system.
  In(event): In<ListenedEvent<Click>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  tool_query: Query<&PlayerTool, With<PLAYERMOVABLE>>,
  //buttons: Res<Input<MouseButton>>,
)-> Bubble{
  //println!("[[[ ======....................");
  //if buttons.pressed(MouseButton::Right){
    //println!("[[                    ====]]");
  //}

  //println!("EVENT: {:?}", buttons.pressed(MouseButton::Right));
  //println!("EVENT: {:?}", event.button);
  //println!("hit: {:?}", event.hit);
  //println!("listener: {:?}", event.listener);
  //println!("pointer_id: {:?}", event.pointer_id);
  //println!("pointer_location: {:?}", event.pointer_location);
  //println!("target: {:?}", event.target);
  if PointerButton::Primary == event.button {

    if let Ok(tool) = tool_query.get_single(){
      //println!("[[Tool   ]]: {:?}", tool.0);
      if tool.0 == format!("build") {
        let mut pos = event.hit.position.unwrap();
        let normal = event.hit.normal.unwrap();

        pos = pos + normal;
        pos.x = pos.x * 10.0;
        pos.y = pos.y * 10.0;
        pos.z = pos.z * 10.0;
        pos = Vec3::floor(pos);
        pos.x = pos.x / 10.0;
        pos.y = pos.y / 10.0;
        pos.z = pos.z / 10.0;
        pos = Vec3::floor(pos);
        if normal.x == -1. {//odd bug?
          pos.x = pos.x + 1.;
        }

        if normal.y == -1. {//odd bug?
          pos.y = pos.y + 1.;
        }

        if normal.z == -1. {//odd bug?
          pos.z = pos.z + 1.;
        }
        let fixed_pos = pos;
        //println!("TOP {:?}",fixed_pos);
        create_cube_physics(&mut commands, &mut meshes, &mut materials, fixed_pos)
        //create_cube_physics(&mut commands, &mut meshes, &mut materials, Vec3::new(fixed_place.x, fixed_place.y, fixed_place.z))
      }
    }
  }

  if PointerButton::Secondary == event.button {
    //remove entity
    //target_commands.despawn();
    commands.entity(event.target).despawn_recursive();
  }

  Bubble::Up // Determines if the event should continue to bubble through the hierarchy.
}
*/
/*
fn update_place_holder_item(
  In(event): In<ListenedEvent<Move>>,
  mut query: Query<&mut Transform, With<PlaceHolder>>,
  tool_query: Query<&PlayerTool, With<PLAYERMOVABLE>>,
)-> Bubble{

  if let Ok(tool) = tool_query.get_single(){
    //println!("[[Tool   ]]: {:?}", tool.0);
    if tool.0 == format!("build") {
      if let Ok(mut entity) = query.get_single_mut(){
        //println!("event: {:?}", event);
        //println!("hit: {:?}", event.hit);
        //let fixed = Vec3::floor(event.hit.position.unwrap()).add(event.hit.normal.unwrap());
        //println!("entity: {:?}", entity);

        let mut pos = event.hit.position.unwrap();
        let normal = event.hit.normal.unwrap();

        pos = pos + normal;
        pos.x = pos.x * 10.0;
        pos.y = pos.y * 10.0;
        pos.z = pos.z * 10.0;
        pos = Vec3::floor(pos);
        pos.x = pos.x / 10.0;
        pos.y = pos.y / 10.0;
        pos.z = pos.z / 10.0;
        pos = Vec3::floor(pos);
        if normal.x == -1. {//odd bug?
          pos.x = pos.x + 1.;
        }

        if normal.y == -1. {//odd bug?
          pos.y = pos.y + 1.;
        }

        if normal.z == -1. {//odd bug?
          pos.z = pos.z + 1.;
        }
        let fixed_pos = pos;
        println!("fixed_pos: {}",fixed_pos);

        entity.translation = fixed_pos;
        //commands.entity(loading_asset_entity).despawn_recursive();
      }
    }
  }
  
  Bubble::Up // Determines if the event should continue to bubble through the hierarchy.
}
*/