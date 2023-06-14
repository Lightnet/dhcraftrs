/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

//===============================================
//
//===============================================

// https://github.com/emilk/egui/issues/496
// https://kerkour.com/rust-error-handling
// https://stackoverflow.com/questions/32384594/how-to-check-whether-a-path-exists

//use fetch_data::sample_file;
//use fetch_data::{ctor, FetchData, FetchDataError};
//use std::path::{Path, PathBuf};
//use std::path::Path;
use eframe::egui;

//fn main() -> Result<(), eframe::Error> {
fn main(){
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    println!("init app");
    //local_file();
    //readme_example1().unwrap();

    //println!("test.md {}", Path::new("test.md").exists());
    //println!("test1.md {}", Path::new("test1.md").exists());

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    //println!("init egui");
    eframe::run_native(
        "Tool Launcher",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
    //println!("End");
}

//pub fn local_file() -> Result<(), FetchDataError> {
    //let path = sample_file("test.md")?;
    //println!("{}", std::fs::metadata(path)?.len()); // Prints 85
    //Ok(())
    //Ok::<(), FetchDataError>(())
//}

//fn readme_example1() -> Result<(), FetchDataError> {
    //use fetch_data::sample_file;
    //println!("init file");

    //let path = sample_file("test.md")?;
    //println!("init file {}", path.to_string_lossy());
    //println!("{}", std::fs::metadata(path)?.len()); // Prints 85
    //Err(FetchDataError);
    //println!("finish file");
    //Ok(())
    //return Err(E);
//}

struct MyApp {
  name: String,
  status: String,
  age: u32,
  progress: f32,
  animate_progress_bar:bool,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      name: "Arthur".to_owned(),
      status: "Status:".to_owned(),
      age: 42,
      progress: 300.0,
      animate_progress_bar:true,
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Lanuch Application");
        //ui.horizontal(|ui| {
          //let name_label = ui.label("Your name: ");
          //ui.text_edit_singleline(&mut self.name)
            //.labelled_by(name_label.id);
        //});
        ui.horizontal(|ui| {
          //let scalar: f32 = 100.0;
          //let animate_progress_bar:bool = false;
          let progress = self.progress / 360.0;
          let progress_bar = egui::ProgressBar::new(progress)
            .show_percentage()
            .animate(true);
            //.animate(*animate_progress_bar);
            //*animate_progress_bar = ui
            ui
            .add(progress_bar)
            .on_hover_text("The progress bar can be animated!")
            .hovered();
        });
        ui.horizontal(|ui| {
            ui.label(format!("Status: {}", self.status));
        });
        ui.horizontal(|ui| {
          //ui.
          if ui.button("Check").clicked() {
              println!("Hello")
          }
          if ui.button("Repair").clicked() {
              
          }
          if ui.button("Start").clicked() {
              
          }
          if ui.button("+").clicked() {
            self.progress += 10.0;
          }

          if ui.button("-").clicked() {
            self.progress -= 10.0;
          }
        });
        //ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
        //if ui.button("Click each year").clicked() {
            //self.age += 1;
        //}
        //ui.label(format!("Hello '{}', age {}", self.name, self.age));
    });
  }
}