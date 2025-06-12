use serde_json::Value;
use std::collections::BTreeMap;

use moonraker::types::subscription::{FilelistChange, SensorUpdateNotification};
use tracing::{debug, info};

#[derive(Debug, Default)]
pub struct Printer {
    pub status: String,
    pub gcode_responses: Vec<String>,
    pub heaters: BTreeMap<String, Heater>,
    pub temperature_sensors: BTreeMap<String, f64>,
}

#[derive(Debug)]
pub struct Heater {
    pub target: f64,
    pub temperature: f64,
}

impl Printer {
    pub(crate) fn receive_klippy_status(&mut self, status: impl Into<String>) {
        let status = status.into();
        debug!("receive_klippy_status {status}");
        self.status = status;
    }

    pub(crate) fn receive_gcode_response(&mut self, resp: String) {
        debug!("receive_gcode_response {resp:?}");
        self.gcode_responses.push(resp);
    }

    pub(crate) fn receive_filelist_change(&mut self, change: FilelistChange) {
        debug!("receive_filelist_change({change:?}");
    }

    pub(crate) fn receive_sensor_update(&mut self, update: SensorUpdateNotification) {
        debug!("receive_sensor_update({update:?}");
    }

    pub(crate) fn receive_status_update(&mut self, update: BTreeMap<String, Value>) {
        for (obj, status) in update {
            match obj.as_str() {
                "toolhead" => {}

                o if o.starts_with("extruder")
                    || o.starts_with("heater_bed")
                    || o.starts_with("heater_generic ") =>
                {
                    let heater = self.heaters.entry(obj.clone()).or_insert_with(|| Heater {
                        target: 0.,
                        temperature: 0.,
                    });

                    if let Some(v) = status.get("target").and_then(|v| v.as_f64()) {
                        heater.target = v;
                    }
                    if let Some(v) = status.get("temperature").and_then(|v| v.as_f64()) {
                        heater.temperature = v;
                    }

                    info!("Heater {}: {:?}", obj, heater);
                }

                o if o.starts_with("temperature_sensor ") => {
                    if let Some(v) = status.get("temperature").and_then(|v| v.as_f64()) {
                        self.temperature_sensors.insert(obj, v);
                    }
                }

                _ => {
                    debug!("unimplmemented receive_status_update({obj} {status:?}");
                }
            }
        }
    }
}
