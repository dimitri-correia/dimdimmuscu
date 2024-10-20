use loco_rs::prelude::*;

use crate::views;

pub async fn render_home(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    views::homepage::home(v)
}

pub fn routes() -> Routes {
    Routes::new().prefix("/").add("/", get(render_home))
}
