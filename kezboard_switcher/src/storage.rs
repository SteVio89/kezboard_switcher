use lazy_static::lazy_static;
use std::{
    fs::{self},
    io,
    path::PathBuf,
    sync::Mutex,
};

use crate::DeviceInformation;

lazy_static! {
    pub static ref DEVICE_STORAGE: Mutex<Vec<DeviceInformation>> = Mutex::new(Vec::new());
}

const STORAGE_FILE_NAME: &str = "device_info.json";

pub fn get_storage_file_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().expect("Failed to get config directory");
    config_dir.push("kezboard_switcher");
    std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    config_dir.push(STORAGE_FILE_NAME);
    config_dir
}

pub fn load_from_file() -> io::Result<()> {
    let path = get_storage_file_path();
    if path.exists() {
        let content = fs::read_to_string(path).expect("Failed to read file");
        let devices: Vec<DeviceInformation> =
            serde_json::from_str(&content).expect("Failed to deserialize devices");
        let mut storage = DEVICE_STORAGE.lock().unwrap();
        *storage = devices;
    }
    Ok(())
}

pub fn save_to_file() -> io::Result<()> {
    let path = get_storage_file_path();
    let storage = DEVICE_STORAGE.lock().unwrap();
    let content = serde_json::to_string(&*storage).expect("Failed to serialize devices");
    fs::write(path, content).expect("Failed to write to file");
    Ok(())
}
