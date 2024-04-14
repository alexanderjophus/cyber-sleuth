use pavex::server::Server;
use server::configuration::Config;
use shuttle_runtime::SecretStore;

mod shuttle_pavex;

#[shuttle_runtime::main]
async fn pavex(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_pavex::ShuttlePavex {
    std::env::set_var(
        "APP_PROFILE",
        secrets
            .get("APP_PROFILE")
            .unwrap_or("development".to_string()),
    );

    let server = setup().await?;

    let shuttle_px = shuttle_pavex::PavexService(server);

    Ok(shuttle_px)
}

async fn setup() -> anyhow::Result<Server> {
    // Load environment variables from a .env file, if it exists.
    let _ = dotenvy::dotenv();

    let config = Config::load(None)?;

    let tcp_listener = anyhow::Context::context(
        config.server.listener().await,
        "Failed to bind the server TCP listener",
    )?;

    let server_builder = Server::new().listen(tcp_listener);

    Ok(server_builder)
}