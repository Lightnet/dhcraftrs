/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;
//use bevy_eventlistener::prelude::*;

pub fn create_entity_cube(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  
  commands
    .spawn(
      PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
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


pub fn create_entity_cube_pick(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  
  commands
    .spawn((
      PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::WHITE.into()),
        //material: materials.add(StandardMaterial {
          //base_color: Color::rgba(0.1, 0.6, 0.6, 0.5),
          //alpha_mode: AlphaMode::Blend,
          //..default()
        //}),
        //transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
      },
      PickableBundle::default(),
      RaycastPickTarget::default(),
      #[allow(dead_code,unused_variables)]
      OnPointer::<Click>::target_commands_mut(|click, target_commands| {
        //if click.target != click.listener && click.button == PointerButton::Secondary {
            //target_commands.despawn();
        //}
        println!("Right Click============================");
      }),
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

#[allow(dead_code,unused_variables)]
pub fn create_entity_cube_physics(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  create_cube_physics(&mut commands, &mut meshes, &mut materials, Vec3::new(0., -1.5, 1.));
  create_cube_physics(&mut commands, &mut meshes, &mut materials, Vec3::new(0., 0., -3.));
}

#[allow(dead_code,unused_variables)]
pub fn create_cube_physics(
  commands: &mut Commands,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<StandardMaterial>>,
  position: Vec3
){
  commands
  .spawn((
    PbrBundle{
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::BLACK.into()),
      ..default()
    },
    #[allow(dead_code,unused_variables)]
    OnPointer::<Click>::target_commands_mut(|click, target_commands| {
      //if click.target != click.listener && click.button == PointerButton::Secondary {
          //target_commands.despawn();
      //}
      println!("Right Click============================");
    }),
  ))
  .insert(Collider::cuboid(0.5, 0.5, 0.5))
  .insert(PickableBundle::default())
  .insert( RaycastPickTarget::default())
  .insert(OnPointer::<Click>::target_commands_mut(|click, target_commands| {
      //if click.target != click.listener && click.button == PointerButton::Secondary {
        //target_commands.despawn();
      //}
      println!("Right Click===[[[ create_entity_cube_physics ]]]===");
  }))
  .insert(TransformBundle::from(Transform::from_xyz(position.x, position.y, position.z)))
  ;
}