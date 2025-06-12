use std::collections::BTreeMap;

use serde_json::Value;

pub type GcodeResponse = String;
pub type StatusUpdate = (BTreeMap<String, Value>, f64);
pub type SensorUpdateNotification = BTreeMap<String, BTreeMap<String, Value>>;

mod filelist;
mod update_manager;

pub use self::filelist::*;
pub use self::update_manager::*;
