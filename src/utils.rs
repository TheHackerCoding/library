use std::path::PathBuf;

use image::DynamicImage;
use rfd::FileDialog;

pub fn audio_file_picker() -> PathBuf {
    FileDialog::new()
        .add_filter("audio", &["wav"])
        .pick_file()
        .unwrap()
}

lazy_static! {
    pub static ref RAW_ICON: Vec<u8> = include_bytes!("static/ico.png").to_vec();
    pub static ref ICON: DynamicImage = image::load_from_memory(&RAW_ICON).unwrap();
}
