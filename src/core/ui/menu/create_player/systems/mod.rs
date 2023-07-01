/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

pub mod interactions;
pub mod layout;


// https://bevyengine.org/examples/ui/text/
/*
pub fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
  for mut text in &mut query {
      let seconds = time.elapsed_seconds();

      // Update the color of the first and only section.
      text.sections[0].style.color = Color::Rgba {
          red: (1.25 * seconds).sin() / 2.0 + 0.5,
          green: (0.75 * seconds).sin() / 2.0 + 0.5,
          blue: (0.50 * seconds).sin() / 2.0 + 0.5,
          alpha: 1.0,
      };
  }
}
*/