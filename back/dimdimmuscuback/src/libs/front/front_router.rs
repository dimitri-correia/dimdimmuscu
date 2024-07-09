use axum::response::Html;
use axum::routing::get;
use axum::Router;

use crate::libs::routers::fallback::fallback;

pub fn front_routes() -> Router {
    Router::new()
        .route("/", get(landing_page))
        .route("/signup", get(signup_page))
        .route("/login", get(login_page))
        .fallback(fallback)
}

pub async fn landing_page() -> Html<&'static str> {
    Html(include_str!("html/landingpage.html"))
}

pub async fn signup_page() -> Html<&'static str> {
    Html(include_str!("html/signuppage.html"))
}

pub async fn login_page() -> Html<&'static str> {
    Html(include_str!("html/loginpage.html"))
}
