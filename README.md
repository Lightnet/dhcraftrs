# dhcraftrs

# License: CC BY-SA

# Created By: Lightnet
https://creativecommons.org/licenses/by-sa/4.0/

# Code Language:
 * Rust 1.69.0 ( https://www.rust-lang.org/ )

# Engine:
 * Bevy Engine 0.10.0  ( https://bevyengine.org/ )

# Plugins:
 * moonshine-save
 * bevy_console
 * bevy-inspector-egui
 * bevy_mod_picking
 * bevy_rapier3d
 * bevy_egui
 * bevy_flycam
 * bevy_asset_loader
 * bevy_transform_gizmo
 * bevy_renet
 * bevy_pkv

# Information: (work in progress...)
  To build crafting and sandbox game base on minecraft. To play test how logic and module componets work in bevy engine in rust langauge.

  Networking will be tested later once the some basic controls, logics, game and other working and building the world.
  
## Network:
  To develop server and client. For players to enjoy the sandbox world. One reason is that to have permission system. To handle users permission as there will be default settings. As well to handle hacking and spam. For editing and playing at the same time. It would required some settings build into logic server.

  Similar to chat group system.

  There will be three more types. Admin, moderator, land/plots owners and players.

## Design:
  To develop module components is not easy as every part is broke up to handle render and logics.

# Bevy Engine:
  If you learn from godot or unity engine they work simalar ways. As well javascript from react, solid and other frame work design. Module components that have common logic. Reason is simple to reduce loop and render by query filter components that are need in run time application to not waste cpu or gpu data.

  It has basic or same logic like start up, mount, unmount, clean up and other simple logic.

  * https://bevy-cheatbook.github.io/

  Note there youtube, github, offical site examples are good start point but note that it might outdate or upate depend on the coder update it. Since it still in development and refine by the coders working hard to develop stable engine.

## logics:
 * resources set up assets
 * state (struct)
 * set up / start up (once run)
 * enter
 * on update
 * exit (clean up)
 * query and filter functions or struct ( loop )
 * component or struct (tag for filter for query)
 * stageless
 * system (loop)
 * commands (for creating entity and other things)
 * assets (loading content data)

  If you every been in the theater they need to set up stage. They would change stage of the scene and setup the next act.

  To handle the scenes is to have stage change in state. There is condition checks for state. It be found in youtube or cheatbook if update.

  Common state are Assets Loading, MainMenu, Loading Game, Game and other states. It depend what kind of states since it use enum in rust language.

  Need to create setting, config, preset contents. As the game needs to finish or fail conditions.

  It required a lot of set up and clean up when change the stages. It depend on the hardware and limits.

```
  -bevy engine
  --empty scene or entity
```

```
Appstate:
-LoadingAssets (default)
-MainMenu 
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


# Design Layout:
```
src
-bin
--testapp.rs (testing setup and logic loading order something...)
--game.rs (testing setup and logic loading order something...)
-menu
--work in progress
-lib.rs (library)
-api.rs (testing / for easy access for struct)
-plugins.rs (testing / setup logic)
-style.rs ( color?)
-systems.rs (testing / functions / set up)
-events.rs ( n/a)

```

# Run Tests:
```
cargo run (testing)
cargo run --bin game (testing)
cargo run --bin editor
cargo run --bin launcher (testing)
cargo run --bin testapp (testing / prototyping)
```
## Network test:

```
cargo run --bin network -- server
cargo run --bin network -- client
```
https://github.com/lucaspoffo/renet/tree/master/bevy_renet


# Cargo params:
```
cargo run --package testlib //param
```

# Cargo watch:
```
cargo install cargo-watch

cargo watch -x run
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
 * https://github.com/NiklasEi/bevy_common_assets
 * https://github.com/NiklasEi/bevy_asset_loader
 * 
 * 
 * 
 * 