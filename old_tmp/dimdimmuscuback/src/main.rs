use shuttle_runtime::SecretStore;

use dimdimmuscuback::libs::routers::main_router::main_router;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    Ok(main_router(secrets).await.into())
}
