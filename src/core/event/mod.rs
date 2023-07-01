/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
// https://bevy-cheatbook.github.io/patterns/manual-event-clear.html
// https://bevy-cheatbook.github.io/programming/events.

// Testing...

use bevy::prelude::*;

pub struct LevelUpEvent(pub Entity);

#[derive(Component, Debug)]
pub struct PlayerXp(pub i64);

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
    eprintln!("Entity {:?} leveled up!", ev.0);
  }
}

pub struct CraftEventPlugin;

impl Plugin for CraftEventPlugin{//main entry point still in testing...
  fn build(&self, app: &mut App){
    app.add_event::<LevelUpEvent>();
    app.add_system(player_level_up);
    app.add_system(debug_levelups);
    app.add_startup_system (create_test_plauer);
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