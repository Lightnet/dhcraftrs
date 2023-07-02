

// https://bevy-cheatbook.github.io/window/icon.html
// https://stackoverflow.com/questions/74586997/how-to-add-a-window-icon-in-bevy

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
//use bevy::window::WindowId;
//use bevy::window::Window;
//use bevy::prelude::NonSend;
use bevy::winit::WinitWindows;
use winit::window::Icon;
//use bevy::window::Window;

pub fn set_window_icon(
    // we have to use `NonSend` here
    winit: NonSend<WinitWindows>,
    //mut window_query: Query<&mut Window>,
    //windows: Res<Window>,
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
) {
    if let Ok((entity, _window)) = windows.get_single(){
        let primary = winit.get_window(entity).unwrap();
        println!("FOUND SCREEN ID");

        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open("my_icon.png")
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
        primary.set_window_icon(Some(icon));
    }
}

//fn main() {
    //App::new()
      //.add_plugins(DefaultPlugins)
      //.add_startup_system(set_window_icon)
      //.run();
//}