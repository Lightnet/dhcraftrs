/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

// https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy/

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{entity::creature::player::components::{PLAYERMOVABLE, PlayerCamera}, api::AppState};

pub struct CraftPhysics3DPlugin;

impl Plugin for CraftPhysics3DPlugin{

  fn build(&self, app: &mut App){

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugin(RapierDebugRenderPlugin::default());

    app.add_startup_system(setup_graphics);
    app.add_startup_system(setup_physics);
    app.add_system(print_ball_altitude);

  }
}

// CHARACTER CONTROLLER TEST
pub struct CraftPhysics3CharacterDPlugin;

impl Plugin for CraftPhysics3CharacterDPlugin{

  fn build(&self, app: &mut App){

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugin(RapierDebugRenderPlugin::default());

    //app.add_startup_system(create_player);


    app.add_system(create_ground.in_schedule(OnEnter(AppState::InGame)));
    app.add_system(create_player01.in_schedule(OnEnter(AppState::InGame)));


    app.add_system(move_player_physics01.in_set(OnUpdate(AppState::InGame)));



    //app.add_system(create_player.in_schedule(OnEnter(AppState::InGame)));
    //app.add_startup_system(create_player);
    //app.add_system(move_player_physics.in_set(OnUpdate(AppState::InGame)));
    //app.add_system(read_result_system.in_set(OnUpdate(AppState::InGame)));

  }
}

#[allow(dead_code)]
fn create_ground(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  /* Create the ground. */
  commands.spawn(
    //NodeBundle{
      //..default()
    //}
    PbrBundle {
      mesh: meshes.add(shape::Plane::from_size(100.0).into()),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      transform: Transform::from_xyz(0.0, 0.0, 0.0),
      ..default()
    }
  )
    .insert(Collider::cuboid(100.0, 0.1, 100.0))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)))
    ;

  // cube
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size:1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
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

}


//test
#[allow(dead_code)]
fn create_player(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  /* Create the ground. */
  commands
    .spawn(Collider::cuboid(100.0, 0.1, 100.0))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

  //https://bevyengine.org/examples/3d/parenting/
  let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 0.2 }));
  let cube_material_handle = materials.add(StandardMaterial {
    base_color: Color::rgb(0.8, 0.7, 0.6),
    ..default()
  });
  /* Create the bouncing ball. */
  /*
  commands
    .spawn(RigidBody::Dynamic)
    .insert(Collider::ball(0.5))
    .insert(Restitution::coefficient(0.7))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
  */
  /*
  commands
    .spawn(
      PbrBundle {
          mesh: cube_handle.clone(),
          material: cube_material_handle.clone(),
          transform: Transform::from_xyz(0.0, 0.0, 0.0),
          ..default()
        }
    );
    */
  //player
  //commands.spawn((1,2,3)); 3 max
  commands
    .spawn(
    PbrBundle {
        mesh: cube_handle.clone(),
        material: cube_material_handle.clone(),
        //transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
    .insert(Collider::ball(0.5))
    .insert(KinematicCharacterController::default())
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
  /*
  commands
    .spawn(
      PbrBundle {
          mesh: cube_handle.clone(),
          material: cube_material_handle.clone(),
          //transform: Transform::from_xyz(0.0, 0.0, 0.0),
          ..default()
        }
    ).with_children(|parent| {
      parent.spawn((
        Collider::ball(0.1),
        KinematicCharacterController::default(),
        TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0))
      ));
    })
    ;
    */

  /*
  commands
  .spawn((
    Collider::ball(0.5),
    KinematicCharacterController::default(),
    TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0))
  ))
    //.spawn(RigidBody::Dynamic)
    //.spawn(Collider::ball(0.5))
    //.spawn(RigidBody::KinematicPositionBased)
    //.insert(Collider::ball(0.5))
    
    //.insert(KinematicCharacterController::default())
    //.insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
    .with_children(|parent| {
      parent.spawn(
        PbrBundle {
          mesh: cube_handle.clone(),
          material: cube_material_handle.clone(),
          //transform: Transform::from_xyz(0.0, 0.0, 0.0),
          ..default()
        }
      );
    })
    ;
    */
}

#[allow(dead_code)]
fn move_player_physics(
  mut controllers: Query<&mut KinematicCharacterController>
){
  for mut controller in controllers.iter_mut() {
    //controller.translation = Some(Vec3::new(1.0, -0.5, 1.0));//too fast
    controller.translation = Some(Vec3::new(0.0, -0.01, 0.0));
  }
}

#[allow(dead_code)]
fn create_player01(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 0.2 }));
  let cube_material_handle = materials.add(StandardMaterial {
    base_color: Color::rgb(0.8, 0.7, 0.6),
    ..default()
  });

  //player entity set up
  //mesh
  //Collider
  //KinematicCharacterController
  //TransformBundle -> set up position

  commands
    .spawn(
    PbrBundle {
        mesh: cube_handle.clone(),
        material: cube_material_handle.clone(),
        //transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
    .insert(PLAYERMOVABLE)
    .insert(Collider::ball(0.8))
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
        transform: Transform::from_xyz(0.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
        },
        PlayerCamera
      ));
    })
    ;

}

#[allow(dead_code)]
fn move_player_physics01(
  input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut query: Query<&mut Transform, With<PLAYERMOVABLE>>,
  mut controllers: Query<&mut KinematicCharacterController>,
){

  //controllers.get_single_mut();
  //for mut controller in controllers.iter_mut() {
    //controller.translation = Some(Vec3::new(1.0, -0.5, 1.0));//too fast
    //controller.translation = Some(Vec3::new(0.0, -0.01, 0.0));
  //}

  let mut controller = controllers.get_single_mut().unwrap();
  //controller.translation = Some(Vec3::new(0.0, -0.01, 0.0));

  let mut entity_transform = query.single_mut();
  controller.translation = Some(Vec3::new(0.0, -0.01, 0.0));

  if input.pressed(KeyCode::Up) {
    let direction = entity_transform.forward();
    controller.translation = Some(direction);
  }
  if input.pressed(KeyCode::Down) {
    let direction = entity_transform.back();
    controller.translation = Some(direction);
  }
  if input.pressed(KeyCode::Left) {
    entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
      0., 1.0 * 0.1, 0.)
    );
  }
  if input.pressed(KeyCode::Right) {
    entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
      0., 1.0 * -0.1, 0.)
    );
  }

  
  

}

#[allow(dead_code)]
fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
  for (entity, output) in controllers.iter() {
      println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
                entity, output.effective_translation, output.grounded);
  }
}


// SIMPLE TEST
pub struct CraftPhysics3DTestPlugin;

impl Plugin for CraftPhysics3DTestPlugin{

  fn build(&self, app: &mut App){

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugin(RapierDebugRenderPlugin::default());

    app.add_startup_system(setup_graphics);
    app.add_startup_system(setup_physics);
    app.add_system(print_ball_altitude);

  }
}

fn setup_graphics(mut commands: Commands) {
  // Add a camera so we can see the debug-render.
  commands.spawn(Camera3dBundle {
      transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..Default::default()
  });
}

fn setup_physics(mut commands: Commands) {
  /* Create the ground. */
  commands
      .spawn(Collider::cuboid(100.0, 0.1, 100.0))
      .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

  /* Create the bouncing ball. */
  commands
      .spawn(RigidBody::Dynamic)
      .insert(Collider::ball(0.5))
      .insert(Restitution::coefficient(0.7))
      .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
  for transform in positions.iter() {
      println!("Ball altitude: {}", transform.translation.y);
  }
}

