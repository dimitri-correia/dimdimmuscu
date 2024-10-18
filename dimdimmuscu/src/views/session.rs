use loco_rs::prelude::*;

use crate::models::_entities::sessions;

/// Render a list view of sessions.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<sessions::Model>) -> Result<Response> {
    format::render().view(v, "session/list.html", data!({"items": items}))
}

/// Render a single session view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &sessions::Model) -> Result<Response> {
    format::render().view(v, "session/show.html", data!({"item": item}))
}

/// Render a session create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "session/create.html", data!({}))
}

/// Render a session edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &sessions::Model) -> Result<Response> {
    format::render().view(v, "session/edit.html", data!({"item": item}))
}
