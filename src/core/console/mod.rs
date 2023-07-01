/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
/*
  Information:
    The use for cheat, testing and debug?
    Admin for network console commands.
    * spawn item
    * teleport?
 */
use bevy::prelude::*;

use clap::Parser;
use bevy_console::{
  //reply,
  AddConsoleCommand,
  ConsoleCommand,
  //ConsoleConfiguration, 
  ConsolePlugin
};

/// Example command
#[derive(Parser, ConsoleCommand)]
#[command(name = "example")]
pub struct ExampleCommand {
  /// Some message
  msg: String,
}

fn example_command(mut log: ConsoleCommand<ExampleCommand>) {
  if let Some(Ok(ExampleCommand { msg: _ })) = log.take() {
    // handle command
  }
}

pub struct ConsoleCraftPlugin;

impl Plugin for ConsoleCraftPlugin{
  fn build(&self, app: &mut App){
    app.add_plugin(ConsolePlugin);
    app.add_console_command::<ExampleCommand, _>(example_command);
  }
}