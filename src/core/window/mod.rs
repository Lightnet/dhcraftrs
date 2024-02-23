

// https://bevy-cheatbook.github.io/window/icon.html
// https://stackoverflow.com/questions/74586997/how-to-add-a-window-icon-in-bevy
//use bevy::winit::WinitWindows;
use bevy::{
  prelude::*,
  //window::PrimaryWindow, 
  winit::WinitWindows
};
use winit::window::Icon;

#[allow(unused_variables)]
pub fn set_window_icon(
  // we have to use `NonSend` here
  //main_window: Query<Entity, With<PrimaryWindow>>,
  windows: NonSend<WinitWindows>,
) {
  //let Some(primary) = windows.get_window(main_window.single()) else {return};

  let (icon_rgba, icon_width, icon_height) = {
    //"icon.ico"
    let image = image::open("my_icon.png")
        .expect("Failed to open icon path")
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };

  let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
  //primary.set_window_icon(Some(icon));
  for window in windows.windows.values() {
    window.set_window_icon(Some(icon.clone()));
    //window.set_window_icon(Some(icon));
  }
}

//fn main() {
  //App::new()
    //.add_plugins(DefaultPlugins)
    //.add_startup_system(set_window_icon)
    //.run();
//}