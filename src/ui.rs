use std::path::PathBuf;

use crate::{
    library::{MidiCategory, MusicFile, SoundCategory},
    utils::audio_file_picker,
    GUI,
};
use eframe::egui::{self, CtxRef, Response, Sense};

impl GUI {
    pub fn selector(&mut self, ctx: &egui::CtxRef, ui: &mut egui::Ui) {
        ui.heading("Library");
        ui.separator();
        if self.state.all_music.len() == 0 {
            let cont = ui.centered_and_justified(|ui| {
                ui.heading("No files!");
            });
            let res = cont.response;
            self.menu(ctx, &res);
        } else {
            self._selector(ctx, ui)
        }
    }

    fn _selector(&mut self, ctx: &egui::CtxRef, ui: &mut egui::Ui) {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            self.state.all_music.iter_mut().for_each(|x| {
                music_ui(x, ui);
            })
        });
        let size = ui.available_size();
        let (_rect, res) = ui.allocate_exact_size(size, Sense::click_and_drag());
        self.menu(ctx, &res);
    }

    fn menu(&mut self, ctx: &egui::CtxRef, res: &Response) {
        let _res = res.clone();

        let mut file: PathBuf = PathBuf::new();
        if self.state.audio_setup == true {
            self.new_music_config(ctx);
        }
        _res.context_menu(|ui| {
            let file_button = ui.button("Add file");
            let folder_button = ui.button("Add folder");

            if file_button.clicked() {
                match audio_file_picker() {
                    Some(file) => {
                        self.state.tmp_music.location = file.to_str().unwrap().clone().to_string();
                        self.state.tmp_music.name =
                            file.file_name().unwrap().to_str().unwrap().to_string();
                        self.state.audio_setup = true;
                        ui.close_menu();
                    }
                    None => println!("huh no file picked"),
                }
            }
        });
    }

    fn new_music_config(&mut self, ctx: &CtxRef) {
        let mut music = &mut self.state.tmp_music;
        //let mut category = &mut music.category;
        let res = egui::Window::new("New Audio")
            .collapsible(true)
            .resizable(true)
            .title_bar(true)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Path");
                    ui.text_edit_singleline(&mut music.location)
                });
                ui.horizontal(|ui| {
                    ui.label("Name");
                    ui.text_edit_singleline(&mut music.name)
                });
                ui.horizontal(|ui| {
                    ui.label("Maker");
                    ui.text_edit_singleline(&mut music.maker)
                });
                ui.horizontal(|ui| {
                    ui.label("Category");
                    egui::ComboBox::from_label("")
                        .selected_text(match music.category {
                            SoundCategory::HiHat => "Hi-Hat".to_string(),
                            SoundCategory::OpenHat => "Open Hat".to_string(),
                            SoundCategory::Bass808 => "808".to_string(),
                            _ => format!("{:?}", music.category),
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut music.category,
                                SoundCategory::HiHat,
                                "Hi-Hat",
                            );
                            ui.selectable_value(&mut music.category, SoundCategory::Kick, "Kick");
                            ui.selectable_value(&mut music.category, SoundCategory::Snare, "Snare");
                            ui.selectable_value(&mut music.category, SoundCategory::Bass, "Bass");
                            ui.selectable_value(&mut music.category, SoundCategory::Bass808, "808");
                            ui.selectable_value(&mut music.category, SoundCategory::FX, "FX");
                            ui.selectable_value(
                                &mut music.category,
                                SoundCategory::OpenHat,
                                "Open Hat",
                            );
                            ui.selectable_value(&mut music.category, SoundCategory::Perc, "Perc");
                            ui.selectable_value(
                                &mut music.category,
                                SoundCategory::Sample,
                                "Sample",
                            );
                            ui.selectable_value(
                                &mut music.category,
                                SoundCategory::OpenHat,
                                "One Shot",
                            );
                            ui.selectable_value(
                                &mut music.category,
                                SoundCategory::Symbol,
                                "Symbol",
                            );
                            ui.selectable_value(&mut music.category, SoundCategory::Vox, "Vox");
                            ui.selectable_value(
                                &mut music.category,
                                SoundCategory::Midi(MidiCategory::Melody),
                                "Midi: Melody",
                            );
                            ui.selectable_value(
                                &mut music.category,
                                SoundCategory::Midi(MidiCategory::HiHatPattern),
                                "Midi: HiHat Pattern",
                            );
                            ui.selectable_value(&mut music.category, SoundCategory::None, "None");
                        })
                });
                if ui.button("Finish").clicked() {
                    self.library.data.files.push(music.clone());
                    self.library.save().unwrap();
                    *music = MusicFile::none();
                    self.state.audio_setup = false;
                }
            });
    }
}

fn music_ui(data: &MusicFile, ui: &mut egui::Ui) {
    let widget = ui.horizontal(|ui| {
        ui.label(format!("{}", data.name));
        ui.add(egui::Separator::default().vertical());
        ui.label(format!("{}", data.maker));
    });

    if widget.response.double_clicked() {
        println!("uhh double clicked");
        open::that_in_background(&data.location);
    }
}
