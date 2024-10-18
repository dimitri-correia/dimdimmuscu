use loco_rs::prelude::*;

use crate::models::_entities::sets;

/// Render a list view of sets.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<sets::Model>) -> Result<Response> {
    format::render().view(v, "set/list.html", data!({"items": items}))
}

/// Render a single set view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &sets::Model) -> Result<Response> {
    format::render().view(v, "set/show.html", data!({"item": item}))
}

/// Render a set create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "set/create.html", data!({}))
}

/// Render a set edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &sets::Model) -> Result<Response> {
    format::render().view(v, "set/edit.html", data!({"item": item}))
}
