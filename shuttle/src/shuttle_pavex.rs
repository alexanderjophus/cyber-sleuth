use server_sdk::{build_application_state, run};
use shuttle_runtime::Error;
use std::net::SocketAddr;

/// A wrapper type for [pavex::server::Server] so we can implement [shuttle_runtime::Service] for it.
pub struct PavexService(pub pavex::server::Server);

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for PavexService {
    /// Takes the router that is returned by the user in their [shuttle_runtime::main] function
    /// and binds to an address passed in by shuttle.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        let application_state = build_application_state().await;

        println!("addr: {:?}", addr);

        let server = self
            .0
            .bind(addr)
            .await
            .expect("Failed to bind the server TCP listener");

        tracing::info!("Starting to listen for incoming requests at {}", addr);

        run(server, application_state).await;

        Ok(())
    }
}

impl From<pavex::server::Server> for PavexService {
    fn from(router: pavex::server::Server) -> Self {
        Self(router)
    }
}

pub type ShuttlePavex = Result<PavexService, Error>;
