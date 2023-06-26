/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

//===============================================
//
//===============================================
// https://bevy-cheatbook.github.io/features/time.html
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct SplashTime {
  /// track when the bomb should explode (non-repeating timer)
  timer: Timer,
}

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin{

  fn build(&self, app: &mut App){
    app.add_startup_system(setup_splash_spawning);
    app.add_system(splash_screen_time);
    app.add_system(spawn_splash);
    
  }
}

fn splash_screen_time(//loop
  mut commands: Commands,
  mut q: Query<(Entity, &mut SplashTime)>,
  time: Res<Time>,
) {
  for (entity, mut splash_timer) in q.iter_mut() {
      // timers gotta be ticked, to work
      splash_timer.timer.tick(time.delta());

      // if it finished, despawn the bomb
      if splash_timer.timer.finished() {
          println!("DELETE splash...");
          commands.entity(entity).despawn();
      }
  }
}

#[derive(Resource)]
pub struct SplashSpawnConfig {
  /// How often to spawn a new bomb? (repeating timer)
  timer: Timer,
}

/// Spawn a new splash in set intervals of time
fn spawn_splash(
  mut commands: Commands,
  time: Res<Time>,
  mut config: ResMut<SplashSpawnConfig>,
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
      // ... other components ...
    ));
  }
}

/// Configure our bomb spawning algorithm
fn setup_splash_spawning(
  mut commands: Commands,
) {
  println!("create splash time..");
  commands.insert_resource(SplashSpawnConfig {
    // create the repeating timer
    //timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
    timer: Timer::new(Duration::from_secs(10), TimerMode::Once),
  })
}
