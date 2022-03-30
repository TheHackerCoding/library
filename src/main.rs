mod library;
mod ui;
mod utils;

use eframe::{
    egui,
    epi::{self, Storage},
};
use library::{Library, MusicFile};
use utils::ICON;

use crate::utils::RAW_ICON;
#[macro_use]
extern crate lazy_static;
pub struct GUI {
    library: Library,
    state: GUIState,
}

struct GUIState {
    audio_setup: bool,
    tmp_music: MusicFile,
    all_music: Vec<MusicFile>,
}

impl Default for GUI {
    fn default() -> Self {
        let mut lib = Library::default();
        Self {
            library: lib,
            state: GUIState {
                audio_setup: false,
                tmp_music: MusicFile::none(),
                // uhh yea i need to do this
                all_music: Library::default().query()
            },
        }
    }
}

impl epi::App for GUI {
    fn name(&self) -> &str {
        "_null"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        // uhh update
        self.state.all_music = self.library.query();
        egui::SidePanel::left("left").show(ctx, |ui| {
            ui.heading("Folders");
            ui.separator();
            ui.painter();
        });
        egui::CentralPanel::default().show(ctx, |ui| self.selector(ctx, ui));
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        self.library.save().unwrap();
    }

    fn on_exit(&mut self) {
        self.library.save().unwrap();
    }
}

fn main() {
    lazy_static::initialize(&RAW_ICON);
    lazy_static::initialize(&ICON);
    let mut app = GUI::default();
    //println!("{:?}", ICON.to_owned().into_bytes());
    let options = eframe::NativeOptions {
        icon_data: Some(epi::IconData {
            rgba: ICON.to_owned().into_bytes(),
            width: ICON.width(),
            height: ICON.height(),
        }),
        drag_and_drop_support: true,
        ..Default::default()
    };
    //let mut native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), options);
}
