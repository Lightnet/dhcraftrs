
# Physics:

# Notes:
 * there are two type KinematicCharacterControllerOutput and KinematicCharacterController query does not seem to work togehter.
 one is translation and other is check collision and ground.

```rs

  let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 0.2 }));
  let cube_material_handle = materials.add(StandardMaterial {
    base_color: Color::rgb(0.8, 0.7, 0.6),
    ..default()
  });
//...
commands //works
    .spawn(
    PbrBundle {
        mesh: cube_handle.clone(),
        material: cube_material_handle.clone(),
        //transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
    .insert(Collider::ball(0.5))
    .insert(KinematicCharacterController::default())
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));//set position
```

```rs
commands
    .spawn(//does not work not attach to object or insert
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
```

