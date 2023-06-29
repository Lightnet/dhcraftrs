/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
// https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy/
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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

