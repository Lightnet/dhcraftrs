#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main()-> Result<(), eframe::Error >{

  println!("init app");

  let options = eframe::NativeOptions {
      initial_window_size: Some(egui::vec2(320.0, 240.0)),
      ..Default::default()
  };
  //println!("init egui");
  eframe::run_native(
      "Tool Web REST Test",
      options,
      Box::new(|_cc| Box::new(MyApp::default())),
  )//return result need to remove error
}

struct MyApp {
  //name: String,
  status: String,
  //age: u32,
  progress: f32,
  //animate_progress_bar:bool,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      //name: "Arthur".to_owned(),
      status: "Status:".to_owned(),
      //age: 42,
      progress: 300.0,
      //animate_progress_bar:true,
    }
  }
}


// https://doc.rust-lang.org/std/keyword.impl.html
#[allow(dead_code)]
impl MyApp {
  
  fn test(){
    println!("Hello");
  }

  fn test02(&self){
    println!("Progress: {}", self.progress);
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Web Application");
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
          //if ui.button("Check").clicked() {
            //println!("Hello")
          //}
          //if ui.button("Repair").clicked() {
          //}
          //if ui.button("Start").clicked() { 
          //}
          //if ui.button("+").clicked() {
            //self.progress += 10.0;
          //}
          //if ui.button("-").clicked() {
            //self.progress -= 10.0;
          //}

          if ui.button("http request echo").clicked() {
            //self.progress -= 10.0;
            // https://stackoverflow.com/questions/71165876/rust-json-method-not-found-in-resultreqwestblockingresponse-reqwester
            //works
            //let body = reqwest::blocking::get("https://www.rust-lang.org").unwrap().text().unwrap();
            //let body = reqwest::blocking::get("http://localhost:3000/api/bevy/echo").unwrap().text().unwrap();

            //let body = reqwest::blocking::get("http://localhost:3000/api/bevy/echo").unwrap().text().unwrap_or("none".to_string());

            //let body = reqwest::blocking::get("http://localhost:3000/api/bevy/echo").unwrap().text().unwrap();
            //println!("body = {:?}", body);

            let response = match reqwest::blocking::get("http://localhost:3000/api/bevy/echo") {
              // Unwraps the response value from the Result
              Ok(r) => r,
          
              // Unwraps the error from the Result and terminates main()
              Err(err) => {
                println!("Request failed: {}", err.to_string());
                return;
              },
            };

            if response.status().is_success() {
              println!("PASS!");
            }else{
              println!("FAIL!"); //nope
            }

            //MyApp::test();
          }

          //if ui.button("test").clicked() {
            //MyApp::test();
          //}

          //if ui.button("test2").clicked() {
            //self.test02();
          //}
        });
        //ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
        //if ui.button("Click each year").clicked() {
            //self.age += 1;
        //}
        //ui.label(format!("Hello '{}', age {}", self.name, self.age));
    });
  }

}

