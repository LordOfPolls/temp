use dioxus::prelude::*;
use tracing::info;

// use sqlx::PgPool;
// use tokio::sync::OnceCell;
//
// pub static PG_POOL: OnceCell<PgPool> = OnceCell::const_new();


#[server(PostServerData)]
pub async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
pub async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
