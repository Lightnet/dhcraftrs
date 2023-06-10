use bevy::prelude::*;

pub fn spawn_main_menu(
  mut commands: Commands,
  assets_server:Res<AssetServer>
){
  let main_menu_entity = build_main_menu(&mut commands, &assets_server);
}

pub fn despawn_main_menu(){}

pub fn build_main_menu(
  commands: &mut Commands,
  assets_server:&Res<AssetServer>
)-> Entity{
  let main_menu_entity = commands.spawn(
    NodeBundle{
      style:Style { 
        size:Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        ..default()
      },
      background_color: Color::RED.into(),
      ..default()
    }
  )
   .id();

  main_menu_entity
}