/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */
 /*

  Testing sub app.
  
  */

// https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html

use bevy::{prelude::*, app::{AppLabel, SubApp}};

#[derive(Resource, Default)]
struct Val(pub i32);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
pub struct ExampleApp;

pub struct CraftSubAppPlugin;

impl Plugin for CraftSubAppPlugin{//main entry point still in testing...
  fn build(&self, app: &mut App){

    // initialize the main app with a value of 0;
    app.insert_resource(Val(10));

    // create a app with a resource and a single schedule
    let mut sub_app = App::empty();

    // add an outer schedule that runs the main schedule
    //sub_app.add_simple_outer_schedule();
    sub_app.insert_resource(Val(100));

    // initialize main schedule
    //sub_app.init_schedule(CoreSchedule::Main);
    sub_app.add_systems(Main, |counter: Res<Val>| {
      // since we assigned the value from the main world in extract
      // we see that value instead of 100
      assert_eq!(counter.0, 10);
    });

    // add the sub_app to the app
    // loop ?
    app.insert_sub_app(ExampleApp, SubApp::new(sub_app, |main_world, sub_app| {
      // extract the value from the main app to the sub app
      //println!("SUB VAL: {:?}", sub_app.world.resource_mut::<Val>().0);
      //println!("MAIN VAL: {:?}", main_world.resource::<Val>().0);
      sub_app.world.resource_mut::<Val>().0 = main_world.resource::<Val>().0;


      //println!("VAL: {:?}", sub_app.world.resource_mut::<Val>().0)
    }));

  }
}


