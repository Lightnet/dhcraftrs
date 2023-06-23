

# 0.10.0:
 * https://www.youtube.com/watch?v=e92clhk0OBk
```rs
app.add_system(load_main_menu.in_schedule(OnEnter(AppState:MainMenu)))
app.add_system(cleanup_main_menu.in_schedule(OnExit(AppState:MainMenu)))

```

```
fn start_game(
  button_query:Query<&Interaction, With<StartGameButton>>,
  next_state:ResMut<NextState<AppState>>,
){
  if button_query.single() == Interaction:Pressed{
    *next_state = NextState(AppState::InGame);
  }
}
```

```
app.add_systems(
  (
    system_a,
    apply_system_beffers,
    system_b,
    //run in order
  ).chain()
)
```


 * https://blog.logrocket.com/5-rust-game-engines-consider-next-project/
```
asset_server.watch_for_changes().unwrap();
```

https://levelup.gitconnected.com/risk-of-rust-part-7-game-development-81870734466



# save and load:
 * https://github.com/bevyengine/bevy/blob/main/examples/scene/scene.rs
 * https://github.com/Zeenobit/moonshine_save
 * 
 * https://crates.io/crates/bevy_pkv
```
c:\Users\<username>\AppData\Roaming\username
```









# refs:
 * https://www.youtube.com/watch?v=5oKEPZ6LbNE&t=52s
 * https://www.youtube.com/watch?v=GqyJl3tamXU
 * https://www.youtube.com/watch?v=RJHdfLAXptQ Coding an Animated Fox in Rust - Bevy Game Engine - ASMR Programming (No Talking)
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 