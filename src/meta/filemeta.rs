
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct FileMeta {
    pub FileSha1: String,
    pub FileName: String,
    pub FileSize: u64,
    pub Location: String,
    pub UploadAt: String,
}

lazy_static! {
    static ref FILEMETAS: Mutex<HashMap<String, FileMeta>> = {
        let mut map: HashMap<String, FileMeta> = HashMap::new();
        map.insert(
            "default".to_string(),
            FileMeta {
                FileSha1: "n".to_string(),
                FileName: "n".to_string(),
                FileSize: 0,
                Location: "n".to_string(),
                UploadAt: "n".to_string(),
            },
        );
        map.remove(&"default".to_string());
        Mutex::new(map)
    };
}

pub fn get_file_meta(filesha1: &String) -> Option<FileMeta> {
    FILEMETAS.lock().unwrap().get(filesha1).clone().cloned()
}

pub fn upload_file_meta(fmeta: &FileMeta) {
    FILEMETAS
        .lock()
        .unwrap()
        .insert(fmeta.FileSha1.clone(), fmeta.clone());
}

pub fn remove_file_meta(filesha1: &String) {
    FILEMETAS.lock().unwrap().remove(filesha1);
}