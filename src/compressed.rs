use crate::library::SoundCategory;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompressedFile {
    pub updated: SystemTime,
    pub data: CompressedFolder,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompressedFolder {
    pub name: String,
    pub data: Vec<CompressedFile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompressedData {
    pub category: SoundCategory,
    pub data: Vec<u8>,
}
