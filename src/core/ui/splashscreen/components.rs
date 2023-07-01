/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

#[derive(Resource)]
pub struct SplashSpawnConfig {
  /// How often to spawn a new bomb? (repeating timer)
  pub timer: Timer,
}

#[derive(Component)]
pub struct SplashTime {
  /// track when the bomb should explode (non-repeating timer)
  pub timer: Timer,
}