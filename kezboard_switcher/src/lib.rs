use serde::{Deserialize, Serialize};

pub mod hid;
pub mod storage;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DeviceConfiguration {
    pub device: DeviceInformation,
    pub keyboard_mapping: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DeviceInformation {
    pub vendor_id: u32,
    pub product_id: u32,
    pub product: String,
    pub manufacturer: String,
}
