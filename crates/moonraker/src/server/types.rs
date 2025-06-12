use std::collections::HashMap;

use eserde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerInfo {
    pub klippy_connected: bool,
    pub klippy_state: ServerKlippyState,
    pub components: Vec<String>,
    pub failed_components: Vec<String>,
    pub registered_directories: Vec<String>,
    pub warnings: Vec<String>,
    pub websocket_count: usize,
    pub moonraker_version: String,
    #[eserde(compat)]
    pub api_version: (usize, usize, usize),
    pub api_version_string: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerKlippyState {
    Disconnected,
    Startup,
    Ready,
    Error,
    Shutdown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    #[eserde(compat)]
    pub config: HashMap<String, HashMap<String, Value>>,
    pub orig: HashMap<String, HashMap<String, String>>,
    pub files: Vec<ConfigFile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigFile {
    pub filename: String,
    pub sections: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemperatureStore {
    #[serde(flatten)]
    pub sensors: HashMap<String, TemperatureSensor>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemperatureSensor {
    pub temperatures: Vec<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub powers: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speeds: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GcodeStore {
    pub gcode_store: Vec<GcodeTracking>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GcodeTracking {
    #[serde(rename = "type")]
    pub command_type: GcodeCommandType,
    pub message: String,
    pub time: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GcodeCommandType {
    Command,
    Response,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogRolloverResult {
    pub rolled_over: Vec<String>,
    /// Map of failed applications to their error message
    pub failed: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Identification {
    pub connection_id: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebsocketId {
    pub websocket_id: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientType {
    ///	A web application like Mainsail and Fluidd.
    Web,
    ///	A mobile application like Mobileraker.
    Mobile,
    ///	A desktop application like OrcaSlicer.
    Desktop,
    ///	An application intended to drive displays like KlipperScreen.
    Display,
    ///	An interactive bot like MoonCord.
    Bot,
    ///	An external extension like Obico.
    Agent,
    ///	Anything that doesn't fit in to the above categories.
    Other,
}
