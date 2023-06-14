use bevy::prelude::*;

//use crate::menu::components::{MainMenu, PlayButton};
use crate::menu::{
  components::*, 
  styles::*,
};

pub fn spawn_main_menu(
  mut commands: Commands,
  assets_server:Res<AssetServer>
){
  let main_menu_entity = build_main_menu(&mut commands, &assets_server);
}

pub fn despawn_main_menu(
  mut commands: Commands,
  main_menu_query:Query<Entity, With<MainMenu>>,
){
  if let Ok(main_menu_entity) = main_menu_query.get_single(){
    commands.entity(main_menu_entity).despawn_recursive();
  }
}

pub fn build_main_menu(
  commands: &mut Commands,
  assets_server:&Res<AssetServer>
)-> Entity{
  let main_menu_entity = commands.spawn(
    (NodeBundle{
      style:Style {
        flex_direction:FlexDirection::Column,
        justify_content:JustifyContent::Center,
        align_items:AlignItems::Center,
        size:Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        gap:Size::new(Val::Px(8.0), Val::Px(8.0)),
        ..default()
      },
      //background_color: Color::RED.into(),
      ..default()
    },
    MainMenu {},
    )
  ).with_children(|parent |{
    //title
    parent.spawn(
      NodeBundle{
        style:Style {
          flex_direction:FlexDirection::Row,
          justify_content:JustifyContent::Center,
          align_items:AlignItems::Center,
          size: Size::new(Val::Px(300.0), Val::Px(120.0)),
          ..default()
        },
        ..default()
      }
    ).with_children(|parent | {
      //image 1
      parent.spawn(ImageBundle{
        style: Style {
          size:Size::new(Val::Px(64.0), Val::Px(64.0)),
          margin:UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
          ..default()
        },
        image:assets_server.load("images/whiteblockblackline.png").into(),
        ..default()
      });
      
      //text
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Test App", 
                TextStyle {
                  font: assets_server.load("fonts/FiraSans-Bold.ttf"),
                  font_size:32.0,
                  color: Color::WHITE,
                })
            ],
            alignment: TextAlignment::Center,
            ..default()
          },
          ..default()
        }
      );

      //image 2
      parent.spawn(ImageBundle{
        style: Style {
          size:Size::new(Val::Px(64.0), Val::Px(64.0)),
          margin:UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
          ..default()
        },
        image:assets_server.load("images/whiteblockblackline.png").into(),
        ..default()
      });
    });

    //play
    parent.spawn((
      ButtonBundle {
        style:BUTTON_STYLE,
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
      },
      PlayButton {}
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Play", 
                TextStyle {
                  font: assets_server.load("fonts/FiraSans-Bold.ttf"),
                  font_size:32.0,
                  color: Color::WHITE,
                })
            ],
            alignment: TextAlignment::Center,
            ..default()
          },
          ..default()
        }
      );
    });
    //quit
    parent.spawn((
      ButtonBundle {
        style:BUTTON_STYLE,
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
      },
      QuitButton {}
    )).with_children(|parent |{
      parent.spawn(
        TextBundle{
          text: Text{
            sections: vec![
              TextSection::new(
                "Quit", 
                TextStyle {
                  font: assets_server.load("fonts/FiraSans-Bold.ttf"),
                  font_size:32.0,
                  color: Color::WHITE,
                })
            ],
            alignment: TextAlignment::Center,
            ..default()
          },
          ..default()
        }
      );
    });
  })
   .id();

  main_menu_entity
}