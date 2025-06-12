mod printer;
pub use printer::*;

use std::{collections::BTreeMap, sync::Arc};

use futures::StreamExt;
use tokio::{
    pin, select,
    sync::{Notify, RwLock},
    task::{self, JoinError, JoinHandle},
};

use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use moonraker::{
    prelude::*,
    types::{
        printer::QueriedPrinterObjects,
        subscription::{FilelistChange, SensorUpdateNotification, StatusUpdate},
    },
};
use tracing::debug;

pub struct Connection {
    pub client: Arc<WsClient>,
    state: Arc<RwLock<Printer>>,
    subscribed_objects: BTreeMap<String, Option<Vec<String>>>,
}
impl Connection {
    pub async fn connect(url: String) -> anyhow::Result<Self> {
        let client = WsClientBuilder::new().build(&url).await?.into();
        let state = Arc::new(RwLock::new(Printer::default()));

        let me = Self {
            client,
            state,
            subscribed_objects: BTreeMap::new(),
        };

        Ok(me)
    }

    pub fn start(&self) -> Handle {
        let state = self.state.clone();
        let client = self.client.clone();

        let stop = Arc::new(Notify::new());
        let signal = stop.clone();

        let hdl = task::spawn(async move {
            // Get all the listeners we need
            pin! {
                let klippy_disconnected = client.listen_klippy_disconnected().await.unwrap();
                let klippy_ready = client.listen_klippy_ready().await.unwrap();
                let klippy_shutdown = client.listen_klippy_shutdown().await.unwrap();
                let gcode_response = client.listen_gcode_response().await.unwrap();
                let status_update = client.listen_status_update().await.unwrap();
                let filelist_changed = client.listen_filelist_changed().await.unwrap();
                let sensor_updates = client.listen_sensor_updates().await.unwrap();
                let proc_stat_update = client.listen_proc_status_update().await.unwrap();
            }

            loop {
                select! {
                    _ = signal.notified() => break,

                    _ = klippy_disconnected.next() => { state.write().await.receive_klippy_status("disconnected"); },
                    _ = klippy_ready.next() =>        { state.write().await.receive_klippy_status("ready");        },
                    _ = klippy_shutdown.next() =>     { state.write().await.receive_klippy_status("shutdown");     },

                    Some(Ok(change)) = filelist_changed.next() => { state.write().await.receive_filelist_change(change); },
                    Some(Ok(update)) = status_update.next() =>  { state.write().await.receive_status_update(update.0);     },
                    Some(Ok(resp)) = gcode_response.next() => { state.write().await.receive_gcode_response(resp); },
                    Some(Ok(sensor)) = sensor_updates.next() => {state.write().await.receive_sensor_update(sensor); },
                    Some(Ok(stat)) = proc_stat_update.next() => {debug!("Proc stat update: {:?}", stat);},
                }
            }
        });

        Handle(stop, hdl)
    }

    async fn update_subscriptions(
        &self,
    ) -> Result<QueriedPrinterObjects, jsonrpsee::core::ClientError> {
        let status = self
            .client
            .subscribe_printer_objects(&self.subscribed_objects)
            .await?;

        self.state
            .write()
            .await
            .receive_status_update(status.status.clone());

        Ok(status)
    }

    pub async fn subscribe_objects(
        &mut self,
        objects: BTreeMap<impl Into<String>, impl Into<Option<Vec<String>>>>,
    ) -> Result<QueriedPrinterObjects, jsonrpsee::core::ClientError> {
        for (obj, fields) in objects {
            self.subscribed_objects.insert(obj.into(), fields.into());
        }
        self.update_subscriptions().await
    }

    pub async fn subscribe_object(
        &mut self,
        object: String,
        fields: Option<Vec<String>>,
    ) -> Result<QueriedPrinterObjects, jsonrpsee::core::ClientError> {
        self.subscribed_objects.insert(object, fields);
        self.update_subscriptions().await
    }

    pub async fn unsubscribe_object(
        &mut self,
        object: String,
    ) -> Result<QueriedPrinterObjects, jsonrpsee::core::ClientError> {
        let _ = self.subscribed_objects.remove(&object);
        self.update_subscriptions().await
    }
}

pub struct Handle(Arc<Notify>, JoinHandle<()>);
impl Handle {
    pub async fn stop(self) -> Result<(), JoinError> {
        self.0.notify_one();
        self.1.await
    }
}
