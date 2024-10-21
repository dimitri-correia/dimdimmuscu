use crate::models::_entities::users;
use loco_rs::{controller::middleware, db, prelude::*};

use crate::views;

pub async fn render_home(
    ViewEngine(v): ViewEngine<TeraView>,
    auth: Option<middleware::auth::ApiToken<users::Model>>,
) -> Result<impl IntoResponse> {
    dbg!(&auth);
    views::homepage::home(v, auth)
}

pub fn routes() -> Routes {
    Routes::new().prefix("/").add("/", get(render_home))
}
