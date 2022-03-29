use eframe::{
    egui::{self, Ui},
    epi,
};
use library::{Folder, Library, MusicFile};

#[path = "library.rs"]
mod library;

struct App {
    library: Library,
}

impl Default for App {
    fn default() -> Self {
        Self {
            library: Library::default(),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Viewer"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let lib = &self.library;
            ui.heading(format!("Updated: {:?}", lib.updated));
            self.library.data.view(ui);
        });
    }
}

fn main() {
    let app = App::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

fn data(ui: &mut Ui) {}

impl Folder {
    fn view(&mut self, ui: &mut Ui) {
        ui.indent("folder", |ui| {
            ui.heading(format!("Name: {:?}", self.name));
            ui.heading(format!("Updated: {:?}", self.updated));
            ui.heading(format!("Location: {:?}", self.location));
            if self.files.len() > 0 {
                self.files.iter_mut().for_each(|x| x.view(ui))
            }

            if self.subfolders.len() > 0 {
                self.subfolders.iter_mut().for_each(|x| x.view(ui))
            }
        });
    }
}

impl MusicFile {
    fn view(&mut self, ui: &mut Ui) {
        ui.indent("music_file", |ui| {
            ui.heading(format!("Name: {:?}", self.name));
            ui.heading(format!("Updated: {:?}", self.updated));
            ui.heading(format!("Location: {:?}", self.location));
            ui.heading(format!("Category: {:?}", self.category));
            ui.heading(format!("Maker: {:?}", self.maker));
            ui.heading(format!("Favorite: {:?}", self.favorite));
        });
    }
}
