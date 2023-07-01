

# LAYOUTS:
 For easy to write down the docs for menu, player and other entities.

 Note there are many layers that are in testing. It will be subject to change.

# AppState:
```rs
pub enum AppState{
  //#[default]
  MainMenu, // use
  //Next,
  InGame, // use, testing...
  EndGame, // not yet
  LoadingGame, // not yet
  //LoadingScene,
  //LoadingWorld,
  //SCENE,
  #[default]//note that if loading error when not first started. when fn use_my_assets error. 
  AssetLoading, // use
  //BootingApp,
  //StartScreen,
  //ErrorScreen,
  OPTIONS, // use
  CREATEPLAYERNAME, // use
  NETWORK, // use
  SERVER, // use
  CLIENT, // use
}
```

# Main Menu:
  * Components
    * MainMenu
    * NewButton
    * PlayButton
    * OnlineButton
    * OptionsButton
    * QuitButton
 * Spawn Menu
   * OnEnter(AppState::MainMenu)
 * Despawn Menu
   * OnExit(AppState::MainMenu)

# Game:
 * Components