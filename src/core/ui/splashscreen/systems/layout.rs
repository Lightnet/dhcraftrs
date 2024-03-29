/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;
use std::time::Duration;

use crate::core::{ui::splashscreen::{components::{SplashTime, SplashSpawnConfig}, styles::get_splash_screen_style}, styles::get_button_text_style};

pub fn splash_screen_time(//loop
  mut commands: Commands,
  mut q: Query<(Entity, &mut SplashTime), With<SplashTime>>,
  time: Res<Time>,
) {
  for (entity, mut splash_timer) in q.iter_mut() {
    // timers gotta be ticked, to work
    splash_timer.timer.tick(time.delta());

    // if it finished, despawn the bomb
    if splash_timer.timer.finished() {
      println!("DELETE splash...");
      //commands.entity(entity).despawn();
      commands.entity(entity).despawn_recursive();
      commands.remove_resource::<SplashSpawnConfig>();
    }
  }
}

pub fn spawn_splash(
  mut commands: Commands,
  //time: Res<Time>,
  //mut config: ResMut<SplashSpawnConfig>,
  asset_server:Res<AssetServer>,
) {
  println!("create splash");
  commands.spawn((
    SplashTime {
      // create the non-repeating fuse timer
      timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
    },
    NodeBundle{
      style:get_splash_screen_style(),
      //background_color: Color::RED.into(),
      ..default()
    }
    // ... other components ...
  )).with_children(|parent |{
    parent.spawn(
      TextBundle{
        text: Text{
          sections: vec![
            TextSection::new(
              "[Splash Screen]", 
              get_button_text_style(&asset_server),
            )
          ],
          //alignment: TextAlignment::Center,
          ..default()
        },
        ..default()
      }
    );
  });
}

/// Configure our bomb spawning algorithm
pub fn setup_splash_spawning(
  mut commands: Commands,
) {
  println!("create splash time..");
  commands.insert_resource(SplashSpawnConfig {
    // create the repeating timer
    //timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
    timer: Timer::new(Duration::from_secs(10), TimerMode::Once),
  })
}

// Spawn a new splash in set intervals of time
/*
pub fn spawn_splash(
  mut commands: Commands,
  time: Res<Time>,
  mut config: ResMut<SplashSpawnConfig>,
  asset_server:Res<AssetServer>,
) {
  // tick the timer
  config.timer.tick(time.delta());

  if config.timer.finished() {
    println!("create splash");
    commands.spawn((
      SplashTime {
        // create the non-repeating fuse timer
        timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
      },
      NodeBundle{
        style:MAIN_MENU_STYLE,
        //background_color: Color::RED.into(),
        ..default()
      }
      // ... other components ...
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "[Splash Screen]", 
                get_button_text_style(&asset_server),
              )
            ],
            alignment: TextAlignment::Center,
            ..default()
          },
          ..default()
        }
      );
    });
  }
}
*/
