extern crate kezboard_switcher;
use std::{
    process,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use kezboard_switcher::{
    hid::HIDManager,
    storage::{load_from_file, save_to_file, DEVICE_STORAGE},
    DeviceInformation,
};

fn main() {
    if let Err(e) = load_from_file() {
        eprintln!("Error loading devices from file: {}", e);
    }

    let (sender, receiver) = mpsc::channel::<DeviceInformation>();
    thread::spawn(move || {
        for received in receiver {
            let mut storage = DEVICE_STORAGE.lock().unwrap();
            if !storage.contains(&received) {
                storage.push(received.clone());
            }
        }
    });
    let running = Arc::new(Mutex::new(true));
    let r = Arc::clone(&running);
    ctrlc::set_handler(move || {
        let mut r = r.lock().unwrap();
        *r = false;
        if let Err(e) = save_to_file() {
            eprintln!("Error saving devices to file: {}", e);
        }
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    if let Err(e) = kezboard_switcher::hid::PlatformHIDManager::initialize(sender) {
        eprintln!("Error initializing HID Manager: {}", e);
    }
}
