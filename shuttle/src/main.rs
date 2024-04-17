use pavex::server::Server;
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

    let _ = dotenvy::dotenv();

    let server = Server::new();

    let shuttle_px = shuttle_pavex::PavexService(server);

    Ok(shuttle_px)
}
