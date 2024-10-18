use loco_rs::prelude::*;

use crate::models::_entities::movements;

/// Render a list view of movements.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<movements::Model>) -> Result<Response> {
    format::render().view(v, "movement/list.html", data!({"items": items}))
}

/// Render a single movement view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &movements::Model) -> Result<Response> {
    format::render().view(v, "movement/show.html", data!({"item": item}))
}

/// Render a movement create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "movement/create.html", data!({}))
}

/// Render a movement edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &movements::Model) -> Result<Response> {
    format::render().view(v, "movement/edit.html", data!({"item": item}))
}
