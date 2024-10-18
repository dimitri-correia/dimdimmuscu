use loco_rs::prelude::*;

use crate::models::_entities::lifts;

/// Render a list view of lifts.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<lifts::Model>) -> Result<Response> {
    format::render().view(v, "lift/list.html", data!({"items": items}))
}

/// Render a single lift view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &lifts::Model) -> Result<Response> {
    format::render().view(v, "lift/show.html", data!({"item": item}))
}

/// Render a lift create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "lift/create.html", data!({}))
}

/// Render a lift edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &lifts::Model) -> Result<Response> {
    format::render().view(v, "lift/edit.html", data!({"item": item}))
}
