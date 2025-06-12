use jsonrpsee::{
    core::{
        ClientError,
        client::{Subscription, SubscriptionClientT},
    },
    ws_client::WsClient,
};

pub mod types;

use self::types::*;

pub type SubscriptionResult<T> = Result<Subscription<T>, ClientError>;

#[allow(async_fn_in_trait)]
pub trait ClientSubscriptionExt<T: SubscriptionClientT = Self>: SubscriptionClientT {
    async fn listen_gcode_response(&self) -> SubscriptionResult<String> {
        self.subscribe_to_method("notify_gcode_response").await
    }

    async fn listen_status_update(&self) -> SubscriptionResult<StatusUpdate> {
        self.subscribe_to_method("notify_status_update").await
    }

    async fn listen_klippy_ready(&self) -> SubscriptionResult<()> {
        self.subscribe_to_method("notify_klippy_ready").await
    }

    async fn listen_klippy_shutdown(&self) -> SubscriptionResult<()> {
        self.subscribe_to_method("notify_klippy_shutdown").await
    }

    async fn listen_klippy_disconnected(&self) -> SubscriptionResult<()> {
        self.subscribe_to_method("notify_klippy_disconnected").await
    }

    async fn listen_filelist_changed(&self) -> SubscriptionResult<FilelistChange> {
        self.subscribe_to_method("notify_filelist_changed").await
    }

    async fn listen_update_response(&self) -> SubscriptionResult<UpdateManagerResponse> {
        self.subscribe_to_method("notify_update_response").await
    }

    async fn listen_sensor_updates(&self) -> SubscriptionResult<SensorUpdateNotification> {
        self.subscribe_to_method("notify_sensor_update").await
    }

    async fn listen_proc_status_update(&self) -> SubscriptionResult<serde_json::Value> {
        self.subscribe_to_method("notify_proc_stat_update").await
    }
}

impl ClientSubscriptionExt for WsClient {}
