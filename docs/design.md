

# Information:
  Prototype build.


# To do List:
 * save and load 
  * scene
 * train system
 * transportion
  * creature
  * vehicle
  * ships
 * network
 * crafting
  * touch
  * table
  * chair
  * bed
 * technologies
  * magic
  * alchemy
  * electric
  * bio
  * rune
  * smith
 * building
 * seige
 * 

## editor / game:
 * res
  * game mode
  * game storage


## Network:
  Networking will be tested later once the some basic controls, logics, game and other working and building the world.

  To develop server and client. For players to enjoy the sandbox world. One required to build permission system. To handle users permission as there will be default settings. As well to handle hacking and spam. For editing and playing at the same time. It would required some settings build into logic server.

  Similar to chat group system.

  There will be three more types. Admin, moderator, land/plots owners and players.

## Notes:
 * Bevy Renet plugin required some thinking.
 * Simple plugin build but no way to handle close session event yet.
 * Conflict with local test config plugin for loading to crash

## Bevy Engine:
  If you learn from godot or unity engine they work simalar ways. As well javascript from react, solid and other frame work design. Module components that have common logic. Reason is simple to reduce loop and render by query filter components that are need in run time application to not waste cpu or gpu data and loop.

  It has basic or same logic like start up, mount, unmount, clean up and other simple logic.

  * https://bevy-cheatbook.github.io/

  Note there youtube, github, offical site examples are good start point but note that it might outdate or upate depend on the writer, coder and dev update it. Since it still in development and refine by the dev and coders working hard to develop stable engine.

## Design:
  To develop module components is not easy as every part is broke up to handle render and logics.

## Logics:
 * resources set up assets
 * state (struct)
 * start up (once run)
 * on enter ( set up )
 * on update ( loop | query)
 * on exit ( clean up )
 * query and filter functions or struct ( loop )
 * component or struct (tag for filter for query)
 * stageless ()
 * system (loop | conditions)
 * commands (for creating entity and other things)
 * assets (loading content data)

  If you every been in the theater they need to set up stage. They would change stage of the scene and setup the next act.

  To handle the scenes is to have stage change in state. There is condition checks for state. It be found in youtube or cheatbook if update.

  Common state are Assets Loading, MainMenu, Loading Game, Game and other states. It depend what kind of states since it use enum in rust language.

  Need to create setting, config, preset contents. As the game needs to finish or fail conditions.

  It required a lot of set up and clean up when change the stages. It depend on the hardware and limits.

```
  -bevy engine
  --state AppState
  --state NetworkState
  --resources PlayerInfo
  --world
  ---entities
  --system (loop)
  ---game mode
  ---player controller
  ---check for event for player death
  ---menu interaction
  --startup (function one trigger for set up)
  ---menu set up
  ---load player data
  ---load assets?
  ---load map?
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
src ( work in progress and subjec to change)
- bin
-- fpsgame.rs (testing first person shooter camera)
-- game.rs (testing and prototype build)
-- network.rs (simple test in case of break next version)
-- launcher.rs (simple application gui test)
-- testscene.rs (simple test)
- core
-- setup builds
-- menu
--- main
- lib.rs (library)
- api.rs (testing / for easy access for struct)
- plugins.rs (testing / setup logic)
- styles.rs ( color?)
- systems.rs (testing / functions / set up)
- events.rs ( n/a)
```