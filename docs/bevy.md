

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