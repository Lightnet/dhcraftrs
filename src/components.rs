/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

// https://google.github.io/comprehensive-rust/structs/field-shorthand.html
use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Player {
  name:String,
}

impl Default for Player {
  fn default() -> Self {
    Player{
      name: String::from("player"),
    }
  }
}


#[derive(Component)]
pub struct Enemy {}

#[derive(Component)]
pub struct Team {}

#[derive(Component)]
pub struct Health {}

#[derive(Component)]
pub struct Armor {}

#[derive(Component)]
pub struct Magic {}

#[derive(Component)]
pub struct Attack {}

#[derive(Component)]
pub struct Defense {}