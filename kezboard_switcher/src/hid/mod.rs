#[cfg(target_os = "macos")]
mod macos;

pub trait HIDManager {
    fn initialize(sender: Sender<DeviceInformation>) -> Result<(), String>;
}

pub trait KeyboardManager {
    fn set_keyboard_layout(layout: &str) -> Result<(), String>;
}

use std::sync::mpsc::Sender;

#[cfg(target_os = "macos")]
pub use macos::MacOSHIDManager as PlatformHIDManager;

use crate::DeviceInformation;
