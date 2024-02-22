// https://github.com/mvlabat/bevy_egui

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Default, Resource)]
struct SharedUiState {
  shared_input: String,
}

#[derive(Event)]
pub struct LevelUpEvent(pub Entity);

#[derive(Component, Debug)]
pub struct PlayerXp(pub i64);

#[derive(Default, Resource, Debug)]
pub struct NetworkDataType(pub String);

fn player_level_up(
  mut ev_levelup: EventWriter<LevelUpEvent>,
  query: Query<(Entity, &PlayerXp)>,
) {
  for (entity, xp) in query.iter() {
    if xp.0 > 1000 {
      ev_levelup.send(LevelUpEvent(entity));
    }
  }
}

fn debug_levelups(
  mut ev_levelup: EventReader<LevelUpEvent>,
) {
  for ev in ev_levelup.iter() {
    println!("Entity {:?} leveled up!", ev.0);
    //eprintln!("Entity {:?} leveled up!", ev.0);
  }
}

fn create_test_plauer(
  mut commands: Commands,
){

  commands.spawn((
    NodeBundle{
      ..default()
    },
    PlayerXp(1000),
  ));
  
}


//===============================================
//
//===============================================
fn main() {
    let mut app:App = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(EguiPlugin);
    app.init_resource::<SharedUiState>();
    app.init_resource::<NetworkDataType>();
    // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
    // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
    app.add_systems(Update, ui_example_system);
    

    app.add_event::<LevelUpEvent>();
    app.add_systems(Update, player_level_up);
    app.add_systems(Update, debug_levelups);
    app.add_systems(Startup, create_test_plauer);

    app.run();
}

fn ui_example_system(
  mut contexts: EguiContexts,
  mut ev_levelup: EventWriter<LevelUpEvent>,
  query: Query<(Entity, &PlayerXp)>,
  mut network_data_type: ResMut<NetworkDataType>,
) {
  egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
    ui.label("world");


    ui.label("Network");

    if network_data_type.0 == String::from(""){
      if ui.button("Server").clicked() {
        println!("server");
        network_data_type.0 = String::from("server");
      }

      if ui.button("Client").clicked() {
        println!("client");
        network_data_type.0 = String::from("client");
      }
    }

    ui.label("Testing");

    // if ui.button("ToggleTest").clicked() {
    //   println!("Toggle");
    // }

    if network_data_type.0 == String::from("server"){
      if ui.button("Server Ping").clicked() {
        println!("server Ping");
      }
    }

    if network_data_type.0 == String::from("client"){
      if ui.button("client Ping").clicked() {
        println!("client Ping");
      }
    }



    // if ui.button("Event1").clicked() {
    //   println!("[[[ Event1 ]]]");
    //   for (entity, xp) in query.iter() {
    //     println!("xp.0: {:?}", xp.0);
    //     if xp.0 > 1000 {
    //       ev_levelup.send(LevelUpEvent(entity));
    //     }
    //   }
    // }

    // if ui.button("Event2").clicked() {
    //   println!("[[[ Event2 ]]]");
    //   for (entity, xp) in query.iter() {
    //     println!("entity: {:?}", entity);
    //     println!("xp.0: {:?}", xp.0);
    //     //if xp.0 > 1000 {
    //       ev_levelup.send(LevelUpEvent(entity));
    //     //}
    //   }
    // }

  });
}

//===============================================
//
//===============================================