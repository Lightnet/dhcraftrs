/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

// for store in user local data

use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::components::PlayerInfo;

pub struct CraftBaseDataPlugin;

impl Plugin for CraftBaseDataPlugin{
  fn build(&self, app: &mut App){
    app.insert_resource(PkvStore::new("dhcraftrs", "playerdata")); // Users\<username>\AppData\Roaming\<dhcraftrs>
    //https://bevy-cheatbook.github.io/programming/res.html
    app.insert_resource(PlayerInfo { 
      name:"Guest".into(),
      idhash:"00".into() 
    });
    app.add_startup_system(setup_player_info_pkv);
  }
}

// check user local storage if exist then load if not create default name
pub fn setup_player_info_pkv(
  mut pkv: ResMut<PkvStore>,
  mut player_info: ResMut<PlayerInfo>,
) {
  if let Ok(username) = pkv.get::<String>("username") {
    info!("Welcome back {username}");
    println!("Welcome back {username}");
    player_info.name = username;

  } else {
    println!("create user!!");
    pkv.set_string("username", "Guest")
        .expect("failed to store username");
    player_info.name = "Guest".into();
    // alternatively, using the slightly less efficient generic api:
    //pkv.set("username", &"Guest".to_string())
        //.expect("failed to store username");
  }
}
