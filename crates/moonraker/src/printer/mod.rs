pub mod types;
use std::collections::BTreeMap;

use jsonrpsee::proc_macros::rpc;
use types::{PrinterInfo, PrinterObjectsList, QueriedPrinterObjects};

#[rpc(client)]
pub trait Printer {
    #[method(name = "printer.info")]
    async fn get_printer_info(&self) -> RpcResult<PrinterInfo>;

    #[method(name = "printer.emergency_stop")]
    async fn emergency_stop(&self) -> RpcResult<String>;

    #[method(name = "printer.restart")]
    async fn restart_printer(&self) -> RpcResult<String>;

    #[method(name = "printer.firmware_restart")]
    async fn firmware_restart(&self) -> RpcResult<String>;

    #[method(name = "printer.objects.list")]
    async fn list_printer_objects(&self) -> RpcResult<PrinterObjectsList>;

    #[method(name = "printer.objects.query", param_kind = map)]
    async fn query_printer_objects(
        &self,
        objects: &BTreeMap<String, Option<Vec<String>>>,
    ) -> RpcResult<QueriedPrinterObjects>;

    #[method(name = "printer.objects.subscribe", param_kind = map)]
    async fn subscribe_printer_objects(
        &self,
        objects: &BTreeMap<String, Option<Vec<String>>>,
    ) -> RpcResult<QueriedPrinterObjects>;
}
