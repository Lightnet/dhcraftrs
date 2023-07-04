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

// System
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
// System
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

// System
#[allow(dead_code,unused_variables)]
pub fn create_entity_cube_physics(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  //buttons: Res<Input<MouseButton>>,
){
  create_cube_physics(&mut commands, &mut meshes, &mut materials,  Vec3::new(0., -1.5, 1.));
  create_cube_physics(&mut commands, &mut meshes, &mut materials,  Vec3::new(0., 0., -3.));
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
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::BLACK.into()),
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
  .insert(PickableBundle::default())
  .insert( RaycastPickTarget::default())
  //.insert(OnPointer::<Click>::target_commands_mut(|click, target_commands| {
      //if click.target != click.listener && click.button == PointerButton::Secondary {
        //target_commands.despawn();
      //}
      //println!("Right Click===[[[ create_entity_cube_physics ]]]===");
  //}))
  .insert(OnPointer::<Click>::run_callback(click_event_test))
  .insert(TransformBundle::from(Transform::from_xyz(position.x, position.y, position.z)))
  ;
}


// BUILD block here?
// https://bevy-cheatbook.github.io/input/mouse.html#mouse-buttons
// trigger once
// System
#[allow(unused_mut, unused_variables)]
fn click_event_test(
  // The first parameter is always the `ListenedEvent`, passed in by the event listening system.
  In(event): In<ListenedEvent<Click>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  //buttons: Res<Input<MouseButton>>,
)-> Bubble{
  //println!("[[[ ======....................");
  //if buttons.pressed(MouseButton::Right){
    //println!("[[                    ====]]");
  //}

  //println!("EVENT: {:?}", buttons.pressed(MouseButton::Right));
  println!("EVENT: {:?}", event.button);
  println!("hit: {:?}", event.hit);
  println!("listener: {:?}", event.listener);
  println!("pointer_id: {:?}", event.pointer_id);
  println!("pointer_location: {:?}", event.pointer_location);
  println!("target: {:?}", event.target);

  Bubble::Up // Determines if the event should continue to bubble through the hierarchy.
}