#[allow(unused_variables, dead_code)]
/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

/*
  Information:
    Idea design to handle scene and entity when scene chnages.
  * game mode set up
  * scene load
  * load map
  * load player
  * scene clean up
  * quest
  * save and load
  * 
 */

// https://bevy-cheatbook.github.io/patterns/manual-event-clear.html
// https://bevy-cheatbook.github.io/programming/events.
// https://bevy-cheatbook.github.io/input/keyboard.html
// https://github.com/bevyengine/bevy/blob/main/examples/input/keyboard_input.rs
use bevy::prelude::*;

#[derive(Event)]
pub struct LevelUpEvent(pub Entity);

#[derive(Event)]
pub struct SwitchCameraEvent(pub String);

#[derive(Component, Debug)]
pub struct PlayerXp(pub i64);
#[allow(dead_code)]
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
#[allow(dead_code)]
fn debug_levelups(
  mut ev_levelup: EventReader<LevelUpEvent>,
) {
  for ev in ev_levelup.read() {
    println!("Entity {:?} leveled up!", ev.0);
    //eprintln!("Entity {:?} leveled up!", ev.0);
  }
}
#[allow(dead_code)]
fn create_test_player(
  mut commands: Commands,
){

  commands.spawn((
    NodeBundle{
      ..default()
    },
    PlayerXp(1000),
  ));
}

#[allow(dead_code)]
fn switch_camera(
  keys: Res<ButtonInput<KeyCode>>,
  mut ev_switch_camera: EventWriter<SwitchCameraEvent>,
){
  if keys.just_pressed(KeyCode::Space) {
    // Space was pressed
    println!("space");
  }
  if keys.just_pressed(KeyCode::KeyC) {
    // C
    //println!("C");
    let hello = String::from("firstperson");
    ev_switch_camera.send(SwitchCameraEvent(hello));
  }
  if keys.just_pressed(KeyCode::KeyV) {
    let hello = String::from("spector");
    ev_switch_camera.send(SwitchCameraEvent(hello));
  }
}

#[allow(dead_code)]
fn switch_camera_event(
  mut ev_switch_camera: EventReader<SwitchCameraEvent>,
){
  for ev in ev_switch_camera.read() {
    println!("Entity {:?} ev_switchCamera!", ev.0);
  }
}


pub struct CraftEventPlugin;

impl Plugin for CraftEventPlugin{//main entry point still in testing...
  fn build(&self, app: &mut App){
    //app.add_event::<LevelUpEvent>();
    //app.add_systems(Update, player_level_up);
    //app.add_systems(Update, debug_levelups);
    //app.add_systems(Startup, create_test_player);

    app.add_event::<SwitchCameraEvent>();
    app.add_systems(Update, switch_camera_event);
    app.add_systems(Update, switch_camera);
  }
}

