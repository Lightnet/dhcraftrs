/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//use std::{fs::File, io::Read};
//use std::io::Write;

use bevy::{
  prelude::*, 
  winit::WinitSettings,
  //tasks::IoTaskPool,
  //utils::Duration
};

use bevy_console::{
  reply,
  AddConsoleCommand,
  ConsoleCommand,
  ConsoleConfiguration, 
  ConsolePlugin
};


use dhcraftrs::test_print;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy_egui::{egui, EguiContext, EguiPlugin};


#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
enum AppState{
  #[default]
  MainMenu,
  InGame,
}

#[derive(SystemSet, PartialEq, Eq, Clone, Hash, Debug)]
enum PhysicsSet{
  Movement,
  CollisionDetection,
}

use clap::Parser;

/// Example command
#[derive(Parser, ConsoleCommand)]
#[command(name = "example")]
struct ExampleCommand {
    /// Some message
    msg: String,
}

fn example_command(mut log: ConsoleCommand<ExampleCommand>) {
    if let Some(Ok(ExampleCommand { msg })) = log.take() {
        // handle command
    }
}

fn main() {

  test_print();

  App::new()
    .add_state::<AppState>()
    .add_plugins(DefaultPlugins)
    // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
    .insert_resource(WinitSettings::desktop_app())
    //.add_plugin(ConsolePlugin)
    //.insert_resource(ConsoleConfiguration {
      // override config here
      //..Default::default()
    //})
    //.add_console_command::<ExampleCommand, _>(example_command)
    //.add_plugin(WorldInspectorPlugin)
    //.add_plugin(EguiPlugin)//fon't need in case of WorldInspectorPlugin
    //.add_startup_system(setup_simple_scene)
    //.add_system(ui_example_system)
    //.add_startup_system(test_save_file)
    //.add_startup_system(test_load_file)
    //button stuff
    .add_startup_system(setup_button)
    .add_system(button_system)
    .run();
}

/// set up a simple 3D scene
fn setup_simple_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });
  // cube
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
  });
  // light
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 1500.0,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..default()
  });
  // camera
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}

/// set up test01
fn setup_base(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){

}


/*
fn ui_example_system(mut egui_context: ResMut<EguiContext>) {
  egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
    ui.label("world");
  });
}

// The new, updated scene data will be saved here so that you can see the changes
const NEW_SCENE_FILE_PATH: &str = "scenes/test_scene.scn.ron";
// https://github.com/bevyengine/bevy/blob/main/examples/scene/scene.rs
fn test_save_file(
  mut commands: Commands,
  server: Res<AssetServer>
){
  //let mut file = File::create("assets/scenes/test_scene.scn.ron").unwrap();

  //file.write_all(b"Hello, world!").unwrap();
  //file.write(b"Hello, world!").unwrap();

  //let mut file = File::create("assets/scenes/test_scene.scn.ron")?;
  //file.write_all(b"Hello, world!")?;

  #[cfg(not(target_arch = "wasm32"))]
  IoTaskPool::get()
      .spawn(async move {
          // Write the scene RON data to file
          File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
              .and_then(|mut file| file.write(b"helloworld"  ))
              .expect("Error while writing scene to file");
      })
      .detach();
}

fn test_load_file(
  mut commands: Commands,
  server: Res<AssetServer>
){
  #[cfg(not(target_arch = "wasm32"))]
  IoTaskPool::get()
      .spawn(async move {
          // Write the scene RON data to file
          let mut contents = String::new();
          File::open(format!("assets/{NEW_SCENE_FILE_PATH}"))
              .and_then(|mut file|  
                file.read_to_string(&mut contents)
              )
              .expect("Error while writing scene to file");
          println!("{}", contents);
      })
      .detach();
}
*/



//===============================================
// TEST BUTTON
//===============================================

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
  mut interaction_query: Query<
      (&Interaction, &mut BackgroundColor, &Children),
      (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color, children) in &mut interaction_query {
      let mut text = text_query.get_mut(children[0]).unwrap();
      match *interaction {
          Interaction::Clicked => {
              text.sections[0].value = "Press".to_string();
              *color = PRESSED_BUTTON.into();
          }
          Interaction::Hovered => {
              text.sections[0].value = "Hover".to_string();
              *color = HOVERED_BUTTON.into();
          }
          Interaction::None => {
              text.sections[0].value = "Button".to_string();
              *color = NORMAL_BUTTON.into();
          }
      }
  }
}

fn setup_button(mut commands: Commands, asset_server: Res<AssetServer>) {
  // ui camera
  commands.spawn(Camera2dBundle::default());
  commands
      .spawn(NodeBundle {
          style: Style {
              size: Size::width(Val::Percent(100.0)),
              align_items: AlignItems::Center,
              justify_content: JustifyContent::Center,
              ..default()
          },
          ..default()
      })
      .with_children(|parent| {
          parent
              .spawn(ButtonBundle {
                  style: Style {
                      size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                      // horizontally center child text
                      justify_content: JustifyContent::Center,
                      // vertically center child text
                      align_items: AlignItems::Center,
                      ..default()
                  },
                  background_color: NORMAL_BUTTON.into(),
                  ..default()
              })
              .with_children(|parent| {
                  parent.spawn(TextBundle::from_section(
                      "Button",
                      TextStyle {
                          font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                          font_size: 40.0,
                          color: Color::rgb(0.9, 0.9, 0.9),
                      },
                  ));
              });
      });
}