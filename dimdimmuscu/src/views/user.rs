use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::_entities::users;

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
    pub birthdate: NaiveDate,
    pub height_in_cm: i32,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
            birthdate: user.birthdate.clone(),
            height_in_cm: user.height_in_cm.clone(),
        }
    }
}
