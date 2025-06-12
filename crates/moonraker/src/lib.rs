mod printer;
mod server;
mod subscriptions;

pub use crate::printer::PrinterClient;
pub use crate::server::ServerClient;

pub mod types {
    pub use crate::printer::types as printer;
    pub use crate::server::types as server;
    pub use crate::subscriptions::types as subscription;
}

pub mod prelude {
    pub use crate::printer::PrinterClient;
    pub use crate::server::ServerClient;

    pub use crate::subscriptions::ClientSubscriptionExt;

    pub use crate::types::server::ClientType;
}
