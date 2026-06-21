use serde::Deserialize;

use alloc::vec::Vec;
use alloc::string::String;

#[derive(Deserialize, Debug)]
pub struct StrapConfig {
    version: String,
    boot_entry: Option<Vec<StrapBootEntry>>,
}

#[derive(Deserialize, Debug)]
pub struct StrapBootEntry {
    default: Option<bool>,
    kernel: String,
    objman: String,
}
