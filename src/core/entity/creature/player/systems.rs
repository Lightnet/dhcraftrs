/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy_mod_picking::prelude::RaycastPickCamera;
use bevy_rapier3d::prelude::*;

use crate::core::api::AppState;
use super::components::{PLAYERMOVABLE, PlayerCamera, IsGround, PlayerTool};

// DEFAULT ?

pub fn create_entity_first_person_player(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
){
  commands
  .spawn(
    PbrBundle{
      mesh: meshes.add(Mesh::from(shape::Capsule {  
        radius:0.9,
        ..default()
      })),
      material: materials.add(StandardMaterial {
        base_color: Color::rgba(0.9, 0.9, 0.9, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..default()
      }),
      ..default()
    }
  )
  .insert(IsGround(true))
  .insert(Name::new("Player"))
  //player tool check
  .insert(PlayerTool("none".to_string()))
  //.insert(PlayerTool("build".to_string()))
  .insert(PLAYERMOVABLE)
  .insert(RigidBody::KinematicPositionBased)
  //.insert(Collider::ball(1.))
  .insert(Collider::capsule(Vec3 { x: 0., y: -0.5, z: 0. },Vec3 { x: 0., y: 0.5, z: 0. } , 1.0))
  .insert(KinematicCharacterController::default())
  .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
  .with_children(|parent|{
    //CAMERA
    parent.spawn((
      Camera3dBundle {
        camera: Camera  { 
          order:1,
          ..default()
        },
        //transform: Transform::from_xyz(0.0, 1., -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        transform: Transform::from_xyz(0.0, 1., -1.0),
        ..Default::default()
      },
      PlayerCamera,
      RaycastPickCamera::default() //when main camera is active and select to update ray cast
    ));

  });
}


// player
// physics
pub fn create_entity_prototype_player(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
){

  commands
    .spawn(
      PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Capsule {  
          radius:0.9,
          ..default()
        })),
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
        //transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
      }
    )
    .insert(IsGround(true))
    //player tool check
    //.insert(PlayerTool("builds".to_string()))
    .insert(PlayerTool("build".to_string()))
    .insert(PLAYERMOVABLE)
    .insert(RigidBody::KinematicPositionBased)
    //.insert(Collider::ball(1.))
    .insert(Collider::capsule(Vec3 { x: 0., y: -0.5, z: 0. },Vec3 { x: 0., y: 0.5, z: 0. } , 1.0))
    .insert(KinematicCharacterController::default())
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
    .with_children(|parent|{
      
      parent.spawn((
        Camera3dBundle {
          camera: Camera  { 
            order:1,
            //priority: 1 ,
            ..default()
          },
        //transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        transform: Transform::from_xyz(0.0, 5., 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
        },
        PlayerCamera,
        RaycastPickCamera::default()//when main camera is active and select to update ray cast
      ));
      
    })
    
    ;
    // https://github.com/aevyrie/bevy_mod_picking/blob/v0.13/examples/event_listener.rs

}
// https://rapier.rs/docs/user_guides/bevy_plugin/character_controller
#[allow(dead_code, unused_variables)]
pub fn move_player_physics01(
  input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut query: Query<(
    &mut Transform, 
    &mut KinematicCharacterController, 
    //&KinematicCharacterControllerOutput,
    //Entity,
    &IsGround
  ), With<PLAYERMOVABLE>>,
){
  //need for player id later to control them later...
  for (
    mut entity_transform,
    mut controller, 
    //controlleroutput, // controller & controlleroutput = does not work?
    //entity
    is_ground
  ) in query.iter_mut() {
    //println!("IsGround: {}", is_ground.0);
    let gravity = Vec3::new(0.0, -0.1, 0.0);
    if input.pressed( KeyCode::W) {
      let direction = entity_transform.forward() * 0.1;
      //controller.translation.apply() ; Some(direction)
      //controller.translation;
      controller.translation = Some(direction + gravity);
    }else if input.pressed( KeyCode::S) {
      let direction = entity_transform.back() * 0.1;
      controller.translation = Some(direction + gravity);
    }else{
      controller.translation = Some(gravity);
    }
  
    if input.pressed( KeyCode::A) {
      entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
        0., 1.0 * 0.1, 0.)
      );
    }
    if input.pressed( KeyCode::D) {
      entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
        0., 1.0 * -0.1, 0.)
      );
    }
  
    if input.pressed(KeyCode::Space) {
      let mut direction = Vec3::ZERO;
      direction.y = 20.;
      entity_transform.translation += time.delta_seconds() * 1.0 * direction;
    }

    //println!("controlleroutput grounded: {}", controlleroutput.grounded);
    //println!("controlleroutput effective_translation: {:?}", controlleroutput.effective_translation);
  }

}


pub fn move_first_person_player_cam(
  time: Res<Time>,
  mut mouse_motion: EventReader<MouseMotion>,
  //mut camera_query: Query<(&Camera, &mut Transform), With<PlayerCamera>>,
  mut camera_query: Query<(&Camera, &mut Transform), With<PlayerCamera>>,
){
  //let (_, mut camera_transform) = camera_query.single_mut();
  //println!("camera_transform: {:}", camera_transform.type_name());

  //entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
        //ev.delta.y * -0.005,0., 0.)
      //);

      //camera_transform.rotate(Quat::from_euler(EulerRot::XYZ,
        //1. * -0.005,0., 0.)
      //);
}

// https://stackoverflow.com/questions/67008987/how-to-move-camera-arbitrary-of-user-input-in-bevy
// https://bevy-cheatbook.github.io/features/camera.html
#[allow(dead_code, unused_variables)]
pub fn move_first_person_player_physics(
  input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut mouse_motion: EventReader<MouseMotion>,
  mut query: Query<(
    &mut Transform, 
    &mut KinematicCharacterController, 
    //&KinematicCharacterControllerOutput,
    //Entity,
    &IsGround
  ), With<PLAYERMOVABLE>>,
  //mut q: Query<&mut Transform, (With<PerspectiveProjection>, With<PlayerCamera>)>,
  //mut q: Query<&mut Transform, With<PlayerCamera>>,
  //mut q: Query<&mut PerspectiveProjection, With<PlayerCamera>>,
  //mut q: Query<&mut Camera, With<PlayerCamera>>,
  //mut q: Query<&mut Transform, With<PlayerCamera>>,
  //mut q: Query<(&mut Camera, &mut Transform, With<PlayerCamera>)>,
  //mut q: Query<&mut Transform, With<PlayerCamera>>,
  //mut camera_query: Query<(&Camera, &mut Transform)>,
){

  //for (mut pan_orbit, mut transform, projection) in query.iter_mut() {


  //let projection = q.single_mut();
  //println!("projection: {:?}", projection);

  //for ( cam ) in q.iter_mut() {
    //println!("CAM: {:?}", cam.type_name());
  //}
  
  //need for player id later to control them later...
  for (
    mut entity_transform,
    mut controller, 
    //controlleroutput, // controller & controlleroutput = does not work?
    //entity
    is_ground
  ) in query.iter_mut() {
    //println!("IsGround: {}", is_ground.0);
    let gravity = Vec3::new(0.0, -0.1, 0.0);
    if input.pressed( KeyCode::W) {
      let direction = entity_transform.forward() * 0.1;
      //controller.translation.apply() ; Some(direction)
      //controller.translation;
      controller.translation = Some(direction + gravity);
    }else if input.pressed( KeyCode::S) {
      let direction = entity_transform.back() * 0.1;
      controller.translation = Some(direction + gravity);
    }else{
      controller.translation = Some(gravity);
    }
  
    if input.pressed( KeyCode::A) {
      //entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
        //0., 1.0 * 0.1, 0.)
      //);
      let direction = entity_transform.left() * 0.1;
      controller.translation = Some(direction + gravity);
    }
    if input.pressed( KeyCode::D) {
      let direction = entity_transform.right() * 0.1;
      controller.translation = Some(direction + gravity);
    }

    for ev in mouse_motion.iter() { //rewrite
      //entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
        //ev.delta.y * -0.001, ev.delta.x * -0.001, 0.)
      //);
      entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
        0., ev.delta.x * -0.005, 0.)
      );
      //if projection {
        //projection.rotate(Quat::from_euler(EulerRot::XYZ,
          //ev.delta.y * -0.005,0., 0.)
        //)
      //}
      
      //entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
        //ev.delta.y * -0.005,0., 0.)
      //);
    }
  
    if input.pressed(KeyCode::Space) {
      let mut direction = Vec3::ZERO;
      direction.y = 20.;
      entity_transform.translation += time.delta_seconds() * 1.0 * direction;
    }

    //println!("controlleroutput grounded: {}", controlleroutput.grounded);
    //println!("controlleroutput effective_translation: {:?}", controlleroutput.effective_translation);
  }

}

// https://rapier.rs/docs/user_guides/bevy_plugin/character_controller
pub fn read_result_system_player(
  mut controllers: Query<(
    Entity, 
    &KinematicCharacterControllerOutput, 
    &mut IsGround
  ), With<PLAYERMOVABLE>>) {

  for (entity,output, mut isground) in controllers.iter_mut() {
      //println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
                //entity, output.effective_translation, output.grounded);
      isground.0 = output.grounded;
  }
}

// https://bevyengine.org/examples/3d/texture/
// https://github.com/bevyengine/bevy/blob/main/examples/3d/transparency_3d.rs
/// Fades the alpha channel of all materials between 0 and 1 over time.
/// Each blend mode responds differently to this:
/// - [`Opaque`](AlphaMode::Opaque): Ignores alpha channel altogether, these materials stay completely opaque.
/// - [`Mask(f32)`](AlphaMode::Mask): Object appears when the alpha value goes above the mask's threshold, disappears
///                when the alpha value goes back below the threshold.
/// - [`Blend`](AlphaMode::Blend): Object fades in and out smoothly.
pub fn fade_transparency(time: Res<Time>, mut materials: ResMut<Assets<StandardMaterial>>) {
  let alpha = (time.elapsed_seconds().sin() / 2.0) + 0.5;
  for (_, material) in materials.iter_mut() {
      material.base_color.set_a(alpha);
  }
}


/*
pub fn create_entity_player0(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
){

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
  ))
  
  .with_children(| parent | {

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
*/

#[allow(dead_code)]
pub fn set_app_state_game(
  mut app_state_next_state:ResMut<NextState<AppState>>,
){
  //app_state_next_state.set(AppState::InGame);
  app_state_next_state.set(AppState::Game);
}

/*
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
*/

/*
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
*/

/*
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
    if input.pressed(KeyCode::Space) {
      let mut direction = Vec3::ZERO;
      direction.y = 5.;
      entity_transform.translation += time.delta_seconds() * 1.0 * direction;
    }

    //entity_transform.translation += time.delta_seconds() * 2.0 * direction;
  //}
}
*/


