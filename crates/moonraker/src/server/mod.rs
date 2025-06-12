pub mod types;

use jsonrpsee::proc_macros::rpc;
use types::{
    ClientType, GcodeStore, Identification, LogRolloverResult, ServerConfig, ServerInfo,
    TemperatureStore, WebsocketId,
};

#[rpc(client)]
pub trait Server {
    #[method(name = "server.info")]
    async fn get_server_info(&self) -> RpcResult<ServerInfo>;

    #[method(name = "server.config")]
    async fn get_server_config(&self) -> RpcResult<ServerConfig>;

    #[method(name="server.temperature_store", param_kind = map)]
    async fn get_temperature_store(&self, include_monitors: bool) -> RpcResult<TemperatureStore>;

    #[method(name = "server.gcode_store", param_kind = map)]
    async fn get_gcode_store(
        &self,
        #[argument(rename = "count")] limit: Option<usize>,
    ) -> RpcResult<GcodeStore>;

    #[method(name = "server.logs.rollover", param_kind = map)]
    async fn rollover_logs(&self, application: Option<&str>) -> RpcResult<LogRolloverResult>;

    #[method(name = "server.restart")]
    async fn restart_server(&self) -> RpcResult<String>;

    #[method(name = "server.connection.identify", param_kind = map)]
    async fn identify_connection(
        &self,
        client_name: &str,
        version: &str,
        #[argument(rename = "type")] client_type: ClientType,
        url: &str,
    ) -> RpcResult<Identification>;

    #[method(name = "server.connection.identify", param_kind = map)]
    async fn authorize_connection_with_access_token(
        &self,
        client_name: &str,
        version: &str,
        #[argument(rename = "type")] client_type: ClientType,
        url: &str,
        access_token: &str,
    ) -> RpcResult<Identification>;

    #[method(name = "server.connection.identify", param_kind = map)]
    async fn authorize_connection_with_api_key(
        &self,
        client_name: &str,
        version: &str,
        #[argument(rename = "type")] client_type: ClientType,
        url: &str,
        api_key: &str,
    ) -> RpcResult<Identification>;

    #[deprecated = "Please use identify_connection instead"]
    #[method(name = "server.websocket.id")]
    async fn get_websocket_id(&self) -> RpcResult<WebsocketId>;
}
