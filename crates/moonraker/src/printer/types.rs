use std::collections::BTreeMap;

use eserde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrinterInfo {
    pub state: KlippyState,
    pub state_message: String,
    pub hostname: String,
    pub klipper_path: String,
    pub python_path: String,
    pub process_id: usize,
    pub user_id: usize,
    pub group_id: usize,
    pub log_file: String,
    #[serde(alias = "configfile")]
    pub config_file: String,
    pub software_version: String,
    pub cpu_info: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrinterObjectsList {
    pub objects: Vec<String>,
}

// One day i'll extend stubgen to be able to do this....
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueriedPrinterObjects {
    #[eserde(compat)]
    pub status: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum KlippyState {
    Startup,
    Ready,
    Error,
    Shutdown,
}
