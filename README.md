# dhcraftrs

# License: CC BY-SA

# Created By: Lightnet
https://creativecommons.org/licenses/by-sa/4.0/

# Code Language:
 * Rust 1.70.0 ( https://www.rust-lang.org/ )

# Engine:
 * Bevy Engine 0.10.0  ( https://bevyengine.org/ )

# Information: (work in progress...)
  Still need to set up stand alone. To build sandbox world. As well some menu set up for test build them.
  
## Network:
  To develop server and client. For player to enjoy the sandbox world. One reason is that to develop permission system. One reason is handle user permission when entering the game to handle hacking and spam. For editing and playing at the same time. It would required some default settings.

  Simalar to chat group system.

## Design:
  To develop module component is not easy as every part is broke up to handle render and logics.

# Bevy Engine:
  If you learn from godot or unity engine they work simalar ways. As well javascript from react, solid and other frame work design. Module components that have common logic. Reason is simple to reduce loop by query filter components that are need in run time application to not waste cpu or gpu data.

  It has basic or same logic like start up, mount, unmount, clean up and other simple logic.

  * https://bevy-cheatbook.github.io/

  Note there youtube, github, offical site examples are good start point but note that it might outdate or upate depend on the coder update it. Since it still in development and refine by the coder working hard to develop stable engine.

## logics:
 * resources set up assets
 * state (struct)
 * set up / start up (once run)
 * enter
 * exit (clean up)
 * query and filter functions or struct ( loop )
 * component or struct (tag for filter for query)
 * stageless
 * system (loop)
 * commands (for creating entity and other things)
 * assets (loading content data)

  To handle the scenes is to have stage. There is condition checks for state. It be found in youtube or cheatbook if update.

  It required a lot of set up and clean up when change the stages.

  For example Godot. Scene can be made into character, buildings, items and other things.

  Well it need to set up manucal since it empty when run. Need to create prefab setting, config, preset contents.

```
  -bevy engine
  --empty scene or entity
```

```
Appstate:
-MainMenu (default)
-Game
-Pause
-InGameMenu
```
```
  App
  -State(Appstate)
  -add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)));
  -add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
```
  Simple menu set up by check app state conditions.

 https://www.youtube.com/watch?v=iW19V3a96tY


# Run Tests:
```
cargo run --bin game
cargo run --bin editor
cargo run --bin launcher


```
## network test:

```
cargo run --bin network -- server
cargo run --bin network -- client
```
https://github.com/lucaspoffo/renet/tree/master/bevy_renet


# Cargo params:
```
cargo run --package testlib //param
```

# Credits:
 * https://www.youtube.com/watch?v=iW19V3a96tY

# References:
 * https://bevy-cheatbook.github.io/setup/bevy-tools.html
 * https://bevyengine.org/
 * https://www.youtube.com/watch?v=iW19V3a96tY
 * https://github.com/frederickjjoubert/bevy-ball-game/tree/Episode-9-Fixes
 * https://www.youtube.com/watch?v=SmqQ_Is9QX8
 * https://www.youtube.com/watch?v=y50VKG1YvPA Splash screens with bevy asset loader
 * 
 * https://github.com/NiklasEi/bevy_common_assets
 * https://github.com/NiklasEi/bevy_asset_loader
 * 
 * 
 * 
 * 