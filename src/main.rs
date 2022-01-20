mod file;

use eframe::{epi, egui};

#[derive(Default)]
struct MyEguiApp {}

impl epi::App for MyEguiApp {
   fn name(&self) -> &str {
       "My egui App"
   }

   fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}

fn main() {
    let app = MyEguiApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
