
```rs
// https://discord.com/channels/691052431525675048/1137149598352281620/1137149598352281620

//find selected y tugs, and move them to match raycast y pos for mouse raycast via vector projection onto tug position
pub fn manage_y_tugs(
    raycast_sources: Query<&RaycastSource<Selectable>>,
    mut y_tugs: Query<(&mut Transform), (With<Selected>, With<y_tug_flag>)>,
) {

    for raycastsource in raycast_sources.iter() {
        if let Some(ray) = raycastsource.ray {
            //println!("raycast origin is {:#?}", ray);
            for (mut tug) in y_tugs.iter_mut() {
                let vec1 = tug.translation;
                let vec2 = ray.origin();
                let vector_projection = (
                    (vec1 * vec2) 
                    /
                    (vec2.length() * vec2.length())
                ) * vec2;
                println!("projecting tug to {:#?}", vector_projection);
                tug.translation.y = vector_projection.y;
            }

        } 
    }
}


fn update_raycast_with_cursor_position(
  mut cursor: EventReader<CursorMoved>,
  mut query: Query<&mut RaycastSource<MyRaycastSet>>,
) {
  for mut pick_source in &mut query {
      // Grab the most recent cursor event if it exists:
      if let Some(cursor_latest) = cursor.iter().last() {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_latest.position);
        //println!("POS: {:?}", cursor_latest.position)//2D screen
        if let Some(ray) = pick_source.ray {
          let vec2 = ray.origin();
          println!("vec2 {:?}",vec2)
        }
      }
  }
}
```