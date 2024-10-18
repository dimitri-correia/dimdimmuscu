use loco_rs::prelude::*;

use crate::models::_entities::muscles;

/// Render a list view of muscles.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<muscles::Model>) -> Result<Response> {
    format::render().view(v, "muscle/list.html", data!({"items": items}))
}

/// Render a single muscle view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &muscles::Model) -> Result<Response> {
    format::render().view(v, "muscle/show.html", data!({"item": item}))
}

/// Render a muscle create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "muscle/create.html", data!({}))
}

/// Render a muscle edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &muscles::Model) -> Result<Response> {
    format::render().view(v, "muscle/edit.html", data!({"item": item}))
}
