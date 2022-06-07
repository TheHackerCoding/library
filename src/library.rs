use crate::compressed::CompressedFile;
use eframe::egui::{self, CollapsingHeader};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::time::SystemTime;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Library {
    pub updated: SystemTime,
    pub data: Folder,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Folder {
    pub updated: SystemTime,
    pub name: String,
    pub location: String,
    pub files: Vec<MusicFile>,
    pub subfolders: Vec<Folder>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MusicFile {
    pub updated: SystemTime,
    pub name: String,
    pub location: String,
    pub category: SoundCategory,
    pub maker: String,
    pub favorite: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SoundCategory {
    HiHat,
    Kick,
    Snare,
    Bass,
    Bass808,
    FX,
    OpenHat,
    Perc,
    Sample,
    OneShot,
    Symbol,
    Vox,
    Midi(MidiCategory),
    Other(String),
    None,
}
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum MidiCategory {
    HiHatPattern,
    Melody,
}

impl Default for Library {
    fn default() -> Self {
        let _path = Self::default_file();
        let path = Path::new(&_path);

        if path.exists() {
            println!("{}", path.display());
            let mut file = File::open(&path).unwrap();
            let mut data = vec![];
            //file.read_exact(&mut data).unwrap()   ;
            file.read_to_end(&mut data).unwrap();
            Library::decode(&data)
        } else {
            fs::create_dir_all(format!("{}{}", Self::default_location(), "library")).unwrap();
            let mut new = Self::new_empty();
            new.save().unwrap();
            new
        }
    }
}

impl Library {
    pub fn default_location() -> String {
        let result: String;
        let username = whoami::username();
        match whoami::platform() {
            whoami::Platform::Windows => {
                result = format!("C:/Users/{}/AppData/Local/_null/", username)
            }
            whoami::Platform::Linux => result = format!("/home/{}/.config/_null/", username),
            whoami::Platform::MacOS => result = format!("/Users/{}/.config/_null/", username),
            _ => panic!("Unknown platform"),
        }
        result
    }

    pub fn default_file() -> String {
        format!("{}{}", Library::default_location(), "lib.null")
    }

    pub fn new_empty() -> Library {
        Library {
            updated: SystemTime::now(),
            data: {
                Folder {
                    updated: SystemTime::now(),
                    name: "root".to_string(),
                    location: format!("{}{}", Library::default_location(), "library"),
                    files: vec![],
                    subfolders: vec![],
                }
            },
        }
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        self.updated = SystemTime::now();
        let data = self.encode();
        fs::write(Library::default_file(), data)?;
        Ok(())
    }

    pub fn decode(data: &[u8]) -> Self {
        println!("decode: {:?}", data);
        let out = bincode::deserialize(data);

        match out {
            Ok(x) => x,
            Err(e) => panic!("{}", *e),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let out = bincode::serialize(self).unwrap();
        println!("encode: {:?}", out);
        out
    }

    pub fn query(&mut self) -> Vec<MusicFile> {
        self.data.query()
    }
}

impl Folder {
    pub fn add_music(&mut self, url: &str, name: &str) {
        let file = MusicFile {
            updated: SystemTime::now(),
            name: name.to_string(),
            location: url.to_string(),
            category: SoundCategory::None,
            maker: whoami::username(),
            favorite: false,
        };
        self.files.push(file);
    }

    pub fn query(&mut self) -> Vec<MusicFile> {
        let mut result: Vec<MusicFile> = vec![];
        let folders = &mut self.subfolders;
        if folders.len() > 0 {
            folders.iter_mut().for_each(|x| result.extend(x.query()));
        }
        result.extend(self.files.to_owned());
        result
    }

    pub fn tree(&mut self, ui: &mut egui::Ui) {
        let folders = &mut self.subfolders;
        let files = &mut self.files;
        ui.collapsing(&self.name, |ui| {
            if folders.len() > 0 {
                folders.iter_mut().for_each(|x| x.tree(ui));
            }
            if files.len() > 0 {
                files.iter_mut().for_each(|x| x.tree_view(ui));
            }
        });
    }

    pub fn compressed(&mut self) -> CompressedFile {}
}

impl MusicFile {
    pub fn none() -> Self {
        Self {
            updated: SystemTime::now(),
            name: String::new(),
            location: String::new(),
            category: SoundCategory::None,
            maker: String::new(),
            favorite: false,
        }
    }

    pub fn tree_view(&mut self, ui: &mut egui::Ui) {
        ui.label(&self.name);
    }
}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
