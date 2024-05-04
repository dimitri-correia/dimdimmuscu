use dimdimmuscuback::libs::routers::main_router::main_router;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    main_router().await
}
